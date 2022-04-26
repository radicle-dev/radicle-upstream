// Copyright © 2022 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

//! Service for reading and publishing events to the event log.

use anyhow::Context as _;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Envelope {
    /// ID of peer that authored the event
    pub peer_id: librad::PeerId,
    /// The Radicle identity that the event is concerned with
    pub identity: radicle_git_ext::Oid,
    pub topic: String,
    pub event: Event,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Event {
    pub r#type: String,
    pub data: serde_json::Value,
}

#[derive(Clone)]
pub struct EventLog {
    peer: crate::peer::Peer,
    git_fetch: crate::git_fetch::Handle,
    /// Semaphore map for serializing updates to events logs by namespace and topic.
    ref_semaphores: crate::semaphore_map::SemaphoreMap<(radicle_git_ext::Oid, String)>,
}

impl EventLog {
    pub fn new(peer: crate::peer::Peer, git_fetch: crate::git_fetch::Handle) -> Self {
        Self {
            peer,
            git_fetch,
            ref_semaphores: crate::semaphore_map::SemaphoreMap::new(),
        }
    }

    /// Get all events published by peers we replicate directly for the given identity and topic.
    ///
    /// The events are in reverse topological oder. This means that for any two events A and B, if
    /// event A references event B, then event A is positioned before B in the returned vector.
    ///
    /// In addition, if we assume that commit timestamps are monotonic, we are guaranteed that if
    /// event A has a more recent timestamp than event B, then event A is positioned before B in the
    /// returned vector.
    pub async fn get(
        &self,
        identity: radicle_git_ext::Oid,
        topic: String,
    ) -> anyhow::Result<Vec<Envelope>> {
        self.peer
            .monorepo_unblock(move |repo| read(&repo, identity, &topic))
            .await
    }

    /// Write an event for this peer for the given identity and topic. Then, publish the event by
    /// pushing it to the identity’s Git seed if one is known.
    pub async fn publish(
        &self,
        identity: radicle_git_ext::Oid,
        topic: &str,
        event: Event,
    ) -> anyhow::Result<()> {
        let envelope = Envelope {
            peer_id: self.peer.librad_peer().peer_id(),
            identity,
            topic: topic.to_string(),
            event,
        };

        let ref_semaphore = self
            .ref_semaphores
            .acquire((identity, topic.to_string()))
            .await;
        self.peer
            .monorepo_unblock({
                let signer = self.peer.librad_peer().signer().clone();
                let topic = topic.to_string();
                move |repo| {
                    write(&repo, &signer, identity, &topic, &envelope)
                        .context("failed to write event")
                }
            })
            .await?;

        self.peer
            .librad_peer()
            .using_storage(move |storage| {
                librad::git::refs::Refs::update(storage, &librad::git::Urn::new(identity))
            })
            .await
            .context("failed to get librad storage")?
            .context("failed to update project refs")?;
        drop(ref_semaphore);

        let pushed = self
            .git_fetch
            .push_event_logs(identity)
            .await
            .context("failed to push event logs")?;
        if !pushed {
            tracing::warn!(%identity, ?topic, "did not push Upstream event logs to seed")
        }
        Ok(())
    }
}

/// Validates all event logs belonging to this identity and returns the reference names of invalid
/// event logs.
///
/// The following things are validated
/// * Every commit message contains a properly encoded event envelope.
/// * Every commit is properly signed by the peer specified in the event envelope.
/// * Every commit without a parent is holds an `init` event.
pub fn validate(
    repo: &git2::Repository,
    identity: radicle_git_ext::Oid,
) -> anyhow::Result<Vec<String>> {
    let remote_refs = repo
        .references_glob(&ref_name(identity, "*", Some("*")))
        .context("failed to list refs")?;
    let own_refs = repo
        .references_glob(&ref_name(identity, "*", None))
        .context("failed to list refs")?;
    let refs = remote_refs.chain(own_refs);
    let mut invalid_refs = vec![];
    for result in refs {
        let reference = result.context("failed to get next reference")?;
        let ref_name = std::str::from_utf8(reference.name_bytes())
            .context("reference name is not valid UTF-8")?;
        if let Err(err) = validate_ref(repo, ref_name) {
            tracing::warn!(?err, %ref_name, "failed to validate event log ref");
            invalid_refs.push(ref_name.to_string())
        };
    }

    Ok(invalid_refs)
}

/// Validate all commits reachable from the commit referenced by `ref_name`.
fn validate_ref(repo: &git2::Repository, ref_name: &str) -> anyhow::Result<()> {
    let mut revwalk = repo.revwalk().context("failed to create revwalk")?;
    revwalk
        .push_ref(ref_name)
        .context("failed to push reference to revwalk")?;
    for result in revwalk {
        let oid = result.context("failed to get next commit")?;
        validate_commit(repo, oid)?;
    }
    Ok(())
}

/// Validate a commit that carries an event.
fn validate_commit(repo: &git2::Repository, oid: git2::Oid) -> anyhow::Result<()> {
    let commit = repo
        .find_commit(oid)
        .context(format!("commit {oid} not found when walking revs"))?;
    let envelope = envelope_from_message(commit.message_bytes()).context(format!(
        "failed to parse commit message as envelope for {oid}"
    ))?;

    let (signature_encoded_bytes, signed) =
        repo.extract_signature(&oid, Some("radicle-ed25519"))?;
    let signature = {
        let signature_encoded =
            std::str::from_utf8(&signature_encoded_bytes).context("invalid signature field")?;
        let signature_bytes =
            base64::decode(signature_encoded).context("invalid base64 signature encoding")?;
        minicbor::decode::<link_crypto::Signature>(&signature_bytes)
            .context("failed to decode signature")?
    };

    if !signature.verify(&signed, envelope.peer_id.as_public_key()) {
        anyhow::bail!("signature could not be verified");
    }

    Ok(())
}

/// Create a commit signed with the `radicle-ed2551` signature scheme and update the given
/// reference.
fn commit_signed<'repo>(
    repo: &'repo git2::Repository,
    signer: &link_crypto::BoxedSigner,
    reference: &str,
    message: &str,
    tree: &git2::Tree,
    parents: &[&git2::Commit],
) -> anyhow::Result<git2::Commit<'repo>> {
    use librad::Signer as _;

    let git_signature = repo.signature().context("failed to get repo signature")?;
    let commit_buffer =
        repo.commit_create_buffer(&git_signature, &git_signature, message, tree, parents)?;

    let signature = signer
        .sign_blocking(commit_buffer.as_ref())
        .context("failed to sign commit")?;
    let signature = link_crypto::Signature::from(signature);
    let signature_bytes = minicbor::to_vec(signature).context("failed to encode signature")?;
    let commit_id = repo.commit_signed(
        commit_buffer
            .as_str()
            .expect("commit buffer is not valid UTF-8"),
        &base64::encode(signature_bytes),
        Some("radicle-ed25519"),
    )?;
    let commit = repo.find_commit(commit_id).context("commit not found")?;
    repo.reference(reference, commit_id, true, "update")?;
    Ok(commit)
}

/// Write an event in an envelope to the monorepo by commiting it to Git ref for the event log
/// associated with the namespace and topic.
fn write(
    repo: &git2::Repository,
    signer: &link_crypto::BoxedSigner,
    identity: radicle_git_ext::Oid,
    topic: &str,
    envelope: &Envelope,
) -> anyhow::Result<()> {
    let log_ref_name = ref_name(identity, topic, None);

    let maybe_prev_commit = match repo.find_reference(&log_ref_name) {
        Ok(log_ref) => {
            let commit_id = log_ref
                .target()
                .context(format!("ref {log_ref_name} has no target"))?;
            let commit = repo.find_commit(commit_id).context("commit not found")?;
            Some(commit)
        },
        Err(err) if err.code() == git2::ErrorCode::NotFound => None,
        Err(err) => {
            return Err(
                anyhow::Error::new(err).context(format!("failed to get reference {log_ref_name}"))
            )
        },
    };
    let tree = match &maybe_prev_commit {
        Some(prev_commit) => prev_commit.tree().context("tree missing from commit")?,
        None => {
            let treebuilder = repo
                .treebuilder(None)
                .context("failed to create treebuilder")?;
            let tree_oid = treebuilder.write().context("failed to write tree")?;
            repo.find_tree(tree_oid).context("tree not found")?
        },
    };

    let message = envelope_to_message(envelope);
    commit_signed(
        repo,
        signer,
        &log_ref_name,
        &message,
        &tree,
        maybe_prev_commit.iter().collect::<Vec<_>>().as_slice(),
    )?;
    Ok(())
}

/// Get all replicated events for the identity and topic. This includes our events from our own
/// event log as well as all event logs from other peers that we replicate.
fn read(
    repo: &git2::Repository,
    identity: radicle_git_ext::Oid,
    topic: &str,
) -> anyhow::Result<Vec<Envelope>> {
    let my_log_ref_name = ref_name(identity, topic, None);
    let remote_log_ref_glob = ref_name(identity, topic, Some("*"));

    let mut revwalk = repo.revwalk().context("failed to create revwalk")?;
    revwalk
        .set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)
        .context("failed to set revwalk sorting")?;

    if let Err(err) = revwalk.push_ref(&my_log_ref_name) {
        // We check the message instead of `err.code()` because the code is
        // `git2::ErrorCode::Generic`.
        if !err.message().ends_with("not found") {
            return Err(anyhow::Error::new(err)
                .context(format!("failed to push ref `{my_log_ref_name}` to revwalk")));
        }
    }
    revwalk.push_glob(&remote_log_ref_glob).context(format!(
        "failed to push glob `{remote_log_ref_glob}` to revwalk"
    ))?;

    let envelopes = revwalk.filter_map(|oid_result| -> Option<anyhow::Result<_>> {
        let go = || -> anyhow::Result<Option<_>> {
            let oid = oid_result.context("failed to get commit from revwalk")?;
            let commit = repo
                .find_commit(oid)
                .context(format!("commit {oid} not found when walking revs"))?;

            let envelope = envelope_from_message(commit.message_bytes()).context(format!(
                "failed to parse commit message as envelope for {oid}"
            ))?;

            if envelope.identity == identity && envelope.topic == topic {
                Ok(Some(envelope))
            } else {
                Ok(None)
            }
        };

        go().transpose()
    });
    envelopes.collect::<Result<Vec<_>, _>>()
}

/// Prefix for event log references excluding the leading `refs/`
pub const REF_PREFIX: &str = "upstream/events.experimental";

/// Return the name of the Git reference that points to the head of the event log identified by the
/// arguments.
fn ref_name(namespace: radicle_git_ext::Oid, topic: &str, peer: Option<&str>) -> String {
    let namespace = librad::git::Urn::new(namespace).encode_id();
    match peer {
        Some(peer) => {
            format!("refs/namespaces/{namespace}/refs/remotes/{peer}/{REF_PREFIX}/{topic}")
        },
        None => {
            format!("refs/namespaces/{namespace}/refs/{REF_PREFIX}/{topic}")
        },
    }
}

const MESSAGE_CONTENT_KEY: &str = "content";
const MESSAGE_CONTENT_TYPE_KEY: &str = "content-type";
const MESSAGE_EVENT_CONTENT_TYPE: &str = "radicle-upstream-event.v1";

fn envelope_from_message(message: &[u8]) -> anyhow::Result<Envelope> {
    let message =
        std::str::from_utf8(message).context("event commit message is not valid UTF-8")?;
    let message_trailers =
        git2::message_trailers_strs(message).context("failed to get message trailers")?;

    let mut content_type = false;
    let mut content: Option<&str> = None;
    for (key, value) in message_trailers.iter() {
        if key == MESSAGE_CONTENT_TYPE_KEY {
            if value == MESSAGE_EVENT_CONTENT_TYPE {
                content_type = true;
            } else {
                anyhow::bail!("invalid content type for event commit message: {value}");
            }
        }
        if key == MESSAGE_CONTENT_KEY {
            content = Some(value)
        }
        if content.is_some() && content_type {
            break;
        }
    }

    if !content_type {
        anyhow::bail!("no content type set for commit message");
    }

    match content {
        Some(content) => Ok(serde_json::from_str::<Envelope>(content)
            .context("failed to parse event envelope from json")?),
        None => anyhow::bail!("no content field in event commit message trailers"),
    }
}

fn envelope_to_message(envelope: &Envelope) -> String {
    let title = format!("radicle upstream event: {}", envelope.event.r#type);
    let content = serde_json::to_string(&envelope).expect("failed to serialize envelope");
    format!("{title}\n\n{MESSAGE_CONTENT_TYPE_KEY}: {MESSAGE_EVENT_CONTENT_TYPE}\n{MESSAGE_CONTENT_KEY}: {content}\n")
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Assert that event envelope parsing fails if the `content-type` and `content` fields are not
    /// set properly.
    #[test]
    fn invalid_message() {
        let content = serde_json::to_string(&Envelope {
            peer_id: librad::PeerId::from(link_crypto::SecretKey::new()),
            identity: git2::Oid::zero().into(),
            topic: "foo".to_string(),
            event: Event {
                r#type: "foo".to_string(),
                data: serde_json::json!({}),
            },
        })
        .unwrap();

        let message = format!(
            "title\n\n{MESSAGE_CONTENT_TYPE_KEY}: {MESSAGE_EVENT_CONTENT_TYPE}\n{MESSAGE_CONTENT_KEY}: {content}"
        );
        assert!(envelope_from_message(message.as_bytes()).is_ok());

        // No content type set
        let message = format!("title\n\n{MESSAGE_CONTENT_KEY}: {content}");
        assert!(envelope_from_message(message.as_bytes()).is_err());

        // No content
        let message =
            format!("title\n\n{MESSAGE_CONTENT_TYPE_KEY}: {MESSAGE_EVENT_CONTENT_TYPE}\n");
        assert!(envelope_from_message(message.as_bytes()).is_err());

        // Invalid content type
        let message = format!(
            "title\n\n{MESSAGE_CONTENT_TYPE_KEY}: foo-bar\n{MESSAGE_CONTENT_KEY}: {content}"
        );
        assert!(envelope_from_message(message.as_bytes()).is_err());
    }

    #[tokio::test]
    async fn publish() {
        let (event_log, test_peer) = new_test_event_log().await;

        let identity = radicle_git_ext::Oid::from(git2::Oid::zero());
        let topic = "asdf";

        let events = event_log.get(identity, topic.to_string()).await.unwrap();
        assert!(events.is_empty());

        let events_to_publish = (1..10u32)
            .map(|data| Event {
                r#type: String::default(),
                data: serde_json::to_value(data).unwrap(),
            })
            .collect::<Vec<_>>();

        for event in &events_to_publish {
            event_log
                .publish(identity, topic, event.clone())
                .await
                .unwrap();
        }

        let events = event_log
            .get(identity, topic.to_string())
            .await
            .unwrap()
            .into_iter()
            .map(|envelope| envelope.event)
            .rev()
            .collect::<Vec<_>>();

        assert_eq!(events, events_to_publish);

        test_peer
            .peer
            .monorepo_unblock(move |repo| {
                assert_eq!(validate(&repo, identity).unwrap(), Vec::<String>::new());
                Ok(())
            })
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn publish_paralell() {
        let (event_log, test_peer) = new_test_event_log().await;

        let identity = radicle_git_ext::Oid::from(git2::Oid::zero());
        let topic = "asdf";

        let events_to_publish = std::iter::repeat(true)
            .take(16)
            .map(|data| Event {
                r#type: String::default(),
                data: serde_json::to_value(data).unwrap(),
            })
            .collect::<Vec<_>>();

        futures::future::join_all(events_to_publish.iter().map({
            let event_log = event_log.clone();
            move |event| {
                let event_log = event_log.clone();
                async move {
                    event_log
                        .publish(identity, topic, event.clone())
                        .await
                        .unwrap();
                }
            }
        }))
        .await;

        let events = event_log
            .get(identity, topic.to_string())
            .await
            .unwrap()
            .into_iter()
            .map(|envelope| envelope.event)
            .collect::<Vec<_>>();

        assert_eq!(events, events_to_publish);

        test_peer
            .peer
            .monorepo_unblock(move |repo| {
                assert_eq!(validate(&repo, identity).unwrap(), Vec::<String>::new());
                Ok(())
            })
            .await
            .unwrap()
    }

    /// Create a new `EventLog` for testing. The `TestPeer` must live until the end of the test.
    /// Otherwise, the temporary directory is destroyed.
    async fn new_test_event_log() -> (EventLog, crate::peer::test::TestPeer) {
        let test_peer = crate::peer::test::TestPeer::new();
        let (git_fetch, _) = crate::git_fetch::create(
            test_peer.peer.clone(),
            vec![],
            std::time::Duration::from_secs(1000),
            &test_peer.store,
        )
        .await
        .unwrap();
        let event_log = EventLog::new(test_peer.peer.clone(), git_fetch);
        (event_log, test_peer)
    }
}
