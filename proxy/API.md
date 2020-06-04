# Radicle Proxy REST API

## Routes

All routes under the REST API respond with JSON and expect request bodies to be encoded as JSON with the header `content-type: application/json` set.

The examples provided use [curl](https://curl.haxx.se/) and the [window.fetch](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch) function available in the JavaScript environments of modern browsers.

### GET /projects

Retrieve a list of known projects. Returns a list of projects.

Returns a list of Projects. (See Entities)

Curl example:

```
$ curl http://localhost:8080/api/v1/projects

[
  {
    "id": "ccb86b0a6860e27d403a09f15ce444c9c6b262dc.git",
    "metadata": {
      "name": "wow",
      "description": "hello",
      "default_branch": "master",
      "img_url": "https://bovid.space/icons/favicon.ico"
    }
  }
]
```

JavaScript example:

```
fetch("http://localhost:8080/api/v1/projects")
  .then((response) => {
    return response.json()
    // [{ "id": "...", "metadata": {...} }]
    })
```

### POST /projects

Add a new project from the filesystem, assigning a librad ID to it.

Request JSON object:

- `path`: Path to the project on the filesystem, relative to the proxy's working directory.
- `metadata`: An object containing metadata about the project.
- `metadata.name`: The given name of the project.
- `metadata.description`: The given description of the project.
- `metadata.default_branch`: The default branch of the git project, ex: `master`.
- `metadata.img_url`: A URL to the image icon used for the project.

Returns a Project. (See Entities).

Status codes:

- [201 Created](https://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html#sec10.2.2): Project assigned a librad ID.
- [400 Bad Request](https://www.w3.org/Protocols/rfc2616/rfc2616-sec10.html#sec10.4.1): The project is already known to the proxy.

Curl example:

```
$ curl -v -X POST -H 'content-type: application/json' http://localhost:8080/api/v1/projects -d '{"path":"..", "metadata":{"name":"radicle-upstream-proxy", "description":"hello world", "img_url":"http://bovid.space/favicon.ico", "default_branch": "master"}}'

{
  "id": "1a7f9a6e1883fc5781f3b1f8dccf21df04e06f11.git",
  "metadata": {
    "name": "who",
    "description": "hello",
    "default_branch": "master",
    "img_url": "http://bovid.space/favicon.ico"
  }
}
```

JavaScript example:

```
fetch("http://localhost:8080/api/v1/projects", {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    path: "/home/$USER/code/radicle-upstream",
    metadata: {
      name: "radicle-upstream",
      description: "Upstream is a cross-platform desktop client for the radicle code collaboration and registry protocols.",
      default_branch: "master",
      img_url: "https://..."
    }
    })
  })
  .then((response) => {
    return response.json()
    // { "id": "...", "metadata": {...} }
  })
```

### GET /projects/<id>

Retrieve information about a particular project by its librad ID.

Returns a Project. (See Entities).

Curl example:

```
$ curl http://localhost:8080/api/v1/projects/da465d3457cc48968d78f66aadd01f8504c363a2.git

{"id":"da465d3457cc48968d78f66aadd01f8504c363a2.git","metadata":{"name":"radicle-upstream-proxy","description":"hello world","default_branch":"master","img_url":"http://bovid.space/favicon.ico"}}
```

### POST /projects/register

Register a project on the Radicle Registry.

Request JSON object:

- `domain_type`: The type of domain under which the project will be registered.
- `domain_id`: ID of the domain under which the project will be registered.
- `project_name`: Name of the project to use in the registry.
- `maybe_coco_id`: Optional librad ID for the project, used for mutual attestation.

Response JSON object:

- `id`: Abridged hexadecimal ID of this transaction on the registry.
- `messages[]`: Array of messages from the registry.
- `messages[].ProjectRegistration`: Information about the project registration.
- `messages[].ProjectRegistration.org_id`: Name of the organization with which the project is registered.
- `messages[].ProjectRegistration.project_name`: Name of the registered project.
- `state`: Information about the state of the transaction.
- `state.type`: A string indicating the state of the transaction. Currently will only ever be `TransactionApplied`.
- `state.block_hash`: The hash of the block the transaction is included in.
- `timestamp`: An object indicating when this transaction was applied.
- `timestamp.secs_since_epoch`: Seconds since epoch to the moment this transaction was applied.
- `timestamp.nanos_since_epoch`: Nanoseconds since epoch to the second this transaction was applied.

Curl example:

```
$ curl -v -X POST -H 'content-type: application/json' http://localhost:8080/api/v1/projects/register -d '{"org_id": "radicle", "project_name": "radicle-upstream"}'

{
  "id": "0x47d8…3fcb",
  "messages": [
    {
      "ProjectRegistration": {
        "org_id": "abcdef",
        "project_name": "cool-kids-club"
      }
    }
  ],
  "state": {
    "type": "TransactionApplied",
    "block_hash": "0x0000…0000"
  },
  "timestamp": {
    "secs_since_epoch": 1585616602,
    "nanos_since_epoch": 216308544
  }
}
```

JavaScript example:

```
fetch("http://localhost:8080/v1/projects/register", {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    org_id: "radicle",
    project_name: "radicle-upstream"
    })
  })
```

## Entities

The API returns some common objects representing entities in the domain.

### Project

- `id`: Librad ID of the project.
- `metadata`: An object containing metadata about the project.
- `metadata.name`: The given name of the project.
- `metadata.description`: The given description of the project.
- `metadata.default_branch`: The default branch of the git project, ex: `master`.
- `metadata.img_url`: A URL to the image icon used for the project.
