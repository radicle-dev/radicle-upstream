interface EmojiAvatar {
  background: {
    r: number;
    g: number;
    b: number;
  };
  emoji: string;
}

interface RemoteAvatar {
  url: string;
}

type Avatar = EmojiAvatar | RemoteAvatar
