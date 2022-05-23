// Copyright © 2021 The Radicle Upstream Contributors
//
// This file is part of radicle-upstream, distributed under the GPLv3
// with Radicle Linking Exception. For full terms see the included
// LICENSE file.

import * as zod from "zod";
import type { Fetcher, RequestOptions } from "./fetcher";
import { Identity, identitySchema } from "./identity";

export interface Metadata {
  name: string;
  defaultBranch: string;
  description: string | null;
  delegates: { [urn: string]: string[] };
}

const metadataSchema: zod.Schema<Metadata> = zod.object({
  name: zod.string(),
  defaultBranch: zod.string(),
  description: zod.string().nullable(),
  delegates: zod.record(zod.array(zod.string())),
});

export interface CreateParams {
  repo: NewRepo | ExistingRepo;
  description?: string;
  defaultBranch: string;
}

export interface Project {
  urn: string;
  metadata: Metadata;
  stats: Stats;
  seed: string | null;
}

const projectSchema: zod.Schema<Project> = zod.object({
  urn: zod.string(),
  metadata: metadataSchema,
  stats: zod.object({
    branches: zod.number(),
    commits: zod.number(),
    contributors: zod.number(),
  }),
  seed: zod.string().url().nullable(),
});

export interface FailedProject {
  urn: string;
  metadata: Metadata;
}

const failedProjectSchema: zod.Schema<FailedProject> = zod.object({
  urn: zod.string(),
  metadata: metadataSchema,
});

export interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

interface NewRepo {
  type: "new";
  path: string;
  name: string;
}

interface ExistingRepo {
  type: "existing";
  path: string;
}

export enum RequestStatus {
  Created = "created",
  Requested = "requested",
  Found = "found",
  Cloning = "cloning",
  Cloned = "cloned",
  Cancelled = "cancelled",
  Failed = "failed",
  TimedOut = "timedOut",
}

export interface Request {
  type: RequestStatus;
  urn: string;
}

const requestSchema = zod.object({
  type: zod.enum([
    RequestStatus.Created,
    RequestStatus.Requested,
    RequestStatus.Found,
    RequestStatus.Cloning,
    RequestStatus.Cloned,
    RequestStatus.Cancelled,
    RequestStatus.Failed,
    RequestStatus.TimedOut,
  ]),
  urn: zod.string(),
});

export interface Peer {
  type: PeerType;
  peerId: string;
  status: PeerReplicationStatus;
}

export enum PeerType {
  Local = "local",
  Remote = "remote",
}

export enum PeerRole {
  Contributor = "contributor",
  Delegate = "delegate",
  Tracker = "tracker",
}

export enum PeerReplicationStatusType {
  NotReplicated = "notReplicated",
  Replicated = "replicated",
}

export interface PeerNotReplicated {
  type: PeerReplicationStatusType.NotReplicated;
}

export interface PeerReplicated {
  type: PeerReplicationStatusType.Replicated;
  role: PeerRole;
  user: Identity;
}

export type PeerReplicationStatus = PeerNotReplicated | PeerReplicated;

const peerSchema: zod.Schema<Peer> = zod.object({
  type: zod.enum([PeerType.Local, PeerType.Remote]),
  peerId: zod.string(),
  status: zod.union([
    zod.object({
      type: zod.literal(PeerReplicationStatusType.NotReplicated),
    }),
    zod.object({
      type: zod.literal(PeerReplicationStatusType.Replicated),
      role: zod.enum([
        PeerRole.Tracker,
        PeerRole.Delegate,
        PeerRole.Contributor,
      ]),
      user: identitySchema,
    }),
  ]),
});

export interface Patch {
  id: string;
  peer: Peer;
  message: string | null;
  commit: string;
  mergeBase: string | null;
}

const patchSchema: zod.ZodSchema<Patch> = zod.object({
  id: zod.string(),
  peer: peerSchema,
  message: zod.string().nullable(),
  commit: zod.string(),
  mergeBase: zod.string().nullable(),
});

export class Client {
  private fetcher: Fetcher;

  public constructor(fetcher: Fetcher) {
    this.fetcher = fetcher;
  }

  public async create(params: CreateParams): Promise<Project> {
    return this.fetcher.fetchOk(
      {
        method: "POST",
        path: "projects",
        body: params,
      },
      projectSchema
    );
  }

  public async get(urn: string, options?: RequestOptions): Promise<Project> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/${urn}`,
        options,
      },
      projectSchema
    );
  }

  public async listFailed(): Promise<FailedProject[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "projects/failed",
      },
      zod.array(failedProjectSchema)
    );
  }

  public async listTracked(): Promise<Project[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "projects/tracked",
      },
      zod.array(projectSchema)
    );
  }

  public async listContributed(): Promise<Project[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "projects/contributed",
      },
      zod.array(projectSchema)
    );
  }

  public async listForUser(userUrn: string): Promise<Project[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/user/${userUrn}`,
      },
      zod.array(projectSchema)
    );
  }
  public async requestsList(): Promise<Request[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/requests`,
      },
      zod.array(requestSchema)
    );
  }

  public async requestCancel(urn: string): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "DELETE",
      path: `projects/requests/${urn}`,
    });
  }

  public async requestSubmit(projectUrn: string): Promise<Request> {
    return this.fetcher.fetchOk(
      {
        method: "PUT",
        path: `projects/requests/${projectUrn}`,
      },
      requestSchema
    );
  }

  public async listPeers(
    projectUrn: string,
    options?: RequestOptions
  ): Promise<Peer[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/${projectUrn}/peers`,
        options,
      },
      zod.array(peerSchema)
    );
  }

  public async peerTrack(urn: string, peerId: string): Promise<boolean> {
    return this.fetcher.fetchOk(
      {
        method: "PUT",
        path: `projects/${urn}/track/${peerId}`,
      },
      zod.boolean()
    );
  }

  public async peerUntrack(urn: string, peerId: string): Promise<boolean> {
    return this.fetcher.fetchOk(
      {
        method: "PUT",
        path: `projects/${urn}/untrack/${peerId}`,
      },
      zod.boolean()
    );
  }

  public async patchList(
    projectUrn: string,
    options?: RequestOptions
  ): Promise<Patch[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/${projectUrn}/patches`,
        options,
      },
      zod.array(patchSchema)
    );
  }

  public async eventPublish(
    projectUrn: string,
    topic: string,
    event: Event
  ): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "PUT",
      path: `projects/${projectUrn}/events/${encodeURIComponent(topic)}`,
      body: event,
    });
  }

  public async eventList(
    projectUrn: string,
    topic: string
  ): Promise<EventEnvelope[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/${projectUrn}/events/${encodeURIComponent(topic)}`,
      },
      zod.array(eventEnvelopeSchema)
    );
  }
}

export interface EventEnvelope<T = Event> {
  peer_id: string;
  event: T;
}

export interface Event {
  type: string;
  data?: unknown;
}

const eventEnvelopeSchema = zod.object({
  peer_id: zod.string(),
  event: zod.object({
    type: zod.string(),
    data: zod.unknown(),
  }),
});
