import * as api from "./api";

// TYPES
export interface EmojiAvatar {
  background: {
    r: number;
    g: number;
    b: number;
  };
  emoji: string;
}

export interface RemoteAvatar {
  url: string;
}

export type Avatar = EmojiAvatar | RemoteAvatar

export enum Usage {
  Any = "any",
  Identity = "identity",
  Org = "org",
}

// EVENTS
export const getAvatar = (usage: Usage, id: string): Promise<EmojiAvatar> =>
  api.get<EmojiAvatar>(`avatars/${id}?usage=${usage}`);
