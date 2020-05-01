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
