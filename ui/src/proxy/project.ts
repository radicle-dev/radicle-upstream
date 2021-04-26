import * as zod from "zod";
import type { Fetcher } from "./fetcher";

export interface Project {
  urn: string;
  shareableEntityIdentifier: string;
  metadata: Metadata;
  stats: Stats;
}

const projectSchema: zod.Schema<Project> = zod.object({
  urn: zod.string(),
  shareableEntityIdentifier: zod.string(),
  metadata: zod.object({
    name: zod.string(),
    defaultBranch: zod.string(),
    description: zod.string().nullable(),
    maintainers: zod.array(zod.string()),
  }),
  stats: zod.object({
    branches: zod.number(),
    commits: zod.number(),
    contributors: zod.number(),
  }),
});

export interface Stats {
  branches: number;
  commits: number;
  contributors: number;
}

export interface Metadata {
  name: string;
  defaultBranch: string;
  description: string | null;
  maintainers: string[];
}

export interface CreateParams {
  repo: NewRepo | ExistingRepo;
  description?: string;
  defaultBranch: string;
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

  async listFailed(): Promise<Project[]> {
    return this.fetcher.fetchOk(
      {
        method: "GET",
        path: "projects/failed",
      },
      zod.array(projectSchema)
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

  async requestCancel(urn: string): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "DELETE",
      path: `projects/requests/${urn}`,
    });
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
