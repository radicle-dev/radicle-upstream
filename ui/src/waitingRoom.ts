// Client representation of proxy waiting room requests, subscriptions, etc.

enum Status {
  Created = "created",
  Requested = "requested",
  Found = "found",
  Cloning = "cloning",
  Cloned = "cloned",
  Cancelled = "cancelled",
  Failed = "failed",
  TimedOut = "timed_out",
}

export interface ProjectRequest {
  type: Status;
  urn: string;
}
