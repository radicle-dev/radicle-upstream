import type { Fetcher, RequestOptions } from "./fetcher";

interface ProjectCreateParams {
  name: string;
  description: string;
  defaultBranch: string;
  fakePeers: string[];
}

export class Control {
  private fetcher: Fetcher;

  constructor(fetcher: Fetcher) {
    this.fetcher = fetcher;
  }

  async reset(options?: RequestOptions): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "GET",
      path: "control/reset",
      options,
    });
  }

  async seal(options?: RequestOptions): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "GET",
      path: "control/seal",
      options,
    });
  }

  async projectCreate(
    params: ProjectCreateParams,
    options?: RequestOptions
  ): Promise<void> {
    return this.fetcher.fetchOkNoContent({
      method: "POST",
      path: "control/create-project",
      body: params,
      options,
    });
  }
}
