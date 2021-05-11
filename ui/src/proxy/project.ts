import * as zod from "zod";
import type { Fetcher } from "./fetcher";

export interface Metadata {
  name: string;
  defaultBranch: string;
  description: string | null;
  maintainers: string[];
}

const metadataSchema: zod.Schema<Metadata> = zod.object({
  name: zod.string(),
  defaultBranch: zod.string(),
  description: zod.string().nullable(),
  maintainers: zod.array(zod.string()),
});

export interface CreateParams {
  repo: NewRepo | ExistingRepo;
  description?: string;
  defaultBranch: string;
}

export interface Project {
  urn: string;
  shareableEntityIdentifier: string;
  metadata: Metadata;
  stats: Stats;
}

const projectSchema: zod.Schema<Project> = zod.object({
  urn: zod.string(),
  shareableEntityIdentifier: zod.string(),
  metadata: metadataSchema,
  stats: zod.object({
    branches: zod.number(),
    commits: zod.number(),
    contributors: zod.number(),
  }),
});

export interface FailedProject {
  urn: string;
  shareableEntityIdentifier: string;
  metadata: Metadata;
}

const failedProjectSchema: zod.Schema<FailedProject> = zod
  .object({
    urn: zod.string(),
    shareableEntityIdentifier: zod.string(),
    metadata: metadataSchema,
  })
  .nonstrict();

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

export interface CheckoutParams {
  peerId?: string;
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

const requestSchema = zod
  .object({
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
  })
  // The API provides some additional fields, that we’re not using yet.
  .nonstrict();

export class Client {
  private fetcher: Fetcher;

  constructor(fetcher: Fetcher) {
    this.fetcher = fetcher;
  }

  async create(params: CreateParams): Promise<Project> {
    return this.fetcher.fetchOk(
      {
        method: "POST",
        path: "projects",
        body: params,
      },
      projectSchema
    );
  }

  async get(urn: string): Promise<Project> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/${urn}`,
      },
      projectSchema
    );
  }

  async listFailed(): Promise<FailedProject[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "projects/failed",
      },
      zod.array(failedProjectSchema)
    );
  }

  async listTracked(): Promise<Project[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "projects/tracked",
      },
      zod.array(projectSchema)
    );
  }

  async listContributed(): Promise<Project[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "projects/contributed",
      },
      zod.array(projectSchema)
    );
  }

  async listForUser(userUrn: string): Promise<Project[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/user/${userUrn}`,
      },
      zod.array(projectSchema)
    );
  }
  async requestsList(): Promise<Request[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: `projects/requests/`,
      },
      zod.array(requestSchema)
    );
  }

  async requestCancel(urn: string): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "DELETE",
      path: `projects/requests/${urn}`,
    });
  }

  async requestSubmit(projectUrn: string): Promise<Request> {
    return this.fetcher.fetchOk(
      {
        method: "PUT",
        path: `projects/requests/${projectUrn}`,
      },
      requestSchema
    );
  }

  async peerTrack(urn: string, peerId: string): Promise<boolean> {
    return this.fetcher.fetchOk(
      {
        method: "PUT",
        path: `projects/${urn}/track/${peerId}`,
      },
      zod.boolean()
    );
  }

  async peerUntrack(urn: string, peerId: string): Promise<boolean> {
    return this.fetcher.fetchOk(
      {
        method: "PUT",
        path: `projects/${urn}/untrack/${peerId}`,
      },
      zod.boolean()
    );
  }

  async checkout(urn: string, params: CheckoutParams): Promise<string> {
    return this.fetcher.fetchOk(
      {
        method: "POST",
        path: `projects/${urn}/checkout`,
        body: params,
      },
      zod.string()
    );
  }
}
