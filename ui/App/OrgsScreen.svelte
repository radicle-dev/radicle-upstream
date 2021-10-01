<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as theGraphApi from "ui/src/org/theGraphApi";
  import * as Org from "ui/src/org";
  import * as ensResolver from "ui/src/org/ensResolver";
  import { push } from "ui/src/router";

  import { Avatar, Badge, CopyableIdentifier, Icon } from "ui/DesignSystem";
  import ScreenLayout from "ui/App/ScreenLayout.svelte";

  let resolvedOrgs: ResolvedOrg[] = [
    {
      owner: {
        type: "gnosis-safe",
        address: "0x15E63259524546050D0E8B7f599D6C0f8d53c672",
        members: [
          {
            ethereumAddress: "0x65A35c9AfEE92d37F3763B9B0a019EbBf1DdbaAd",
          },
          {
            ethereumAddress: "0x7BCf7A112e9fC7Ff2b54a52990976E5C3352F2D2",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x7bcf7a112e9fc7ff2b54a52990976e5c3352f2d2",
        id: "0x5234a2e25b5b9802565a53e1859dee3a226def8f",
        owner: "0x15e63259524546050d0e8b7f599d6c0f8d53c672",
        timestamp: 1628247093,
        registration: {
          domain: "community.radicle.eth",
          url: "https://radicle.community",
          avatar:
            "https://pbs.twimg.com/profile_images/1372563232850870274/aREQff_C_400x400.jpg",
          owner: "0x641eDB587867a703E132a4210d63e93C52AA97fF",
          address: "0x5234A2E25B5b9802565a53e1859deE3A226Def8F",
          twitter: "radicle",
          github: "radicle-dev",
          seedId: "hybz7p93bzo5z3qw97g5pbk1f1ue6hrmcqzp1t7hedaoehbu44ixqq",
          seedHost: "seed.radicle.community",
        },
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0x0E5eB59A8f7BDb86BD1a083EcE118A0D1c11181e",
        members: [
          {
            ethereumAddress: "0x9887D70d026818CA841BE7d41AE1AdACc5cB9903",
          },
          {
            ethereumAddress: "0x53bcFaEd43441C7bB6149563eC11f756739C9f6A",
          },
          {
            ethereumAddress: "0x5eB0691cA42D0D8bA102302aD21217817bc5eF80",
          },
          {
            ethereumAddress: "0x5Ebda6C2Ca3970911F46557a322dDd3d3a8718d3",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x53bcfaed43441c7bb6149563ec11f756739c9f6a",
        id: "0x793d577ce86b259f898fecb37b9e77e9cb74a964",
        owner: "0x0e5eb59a8f7bdb86bd1a083ece118a0d1c11181e",
        timestamp: 1626450698,
        registration: {
          domain: "gnosisguild.radicle.eth",
          url: null,
          avatar: null,
          owner: "0x53bcFaEd43441C7bB6149563eC11f756739C9f6A",
          address: "0x793D577cE86B259F898FEcB37b9e77e9Cb74a964",
          twitter: "gnosisguild",
          github: "gnosisguild",
          seedId: null,
          seedHost: null,
        },
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x49E53Fb3d5bf1532fEBAD88a1979E33A94844d1d",
      },
      org: {
        creator: "0x49e53fb3d5bf1532febad88a1979e33a94844d1d",
        id: "0x7c78a8fe3eb35a6194760196ead33063fae8e33b",
        owner: "0x49e53fb3d5bf1532febad88a1979e33a94844d1d",
        timestamp: 1626783172,
        registration: {
          domain: "eth.radicle.eth",
          url: "https://ethereum.org/en/",
          avatar: "https://avatars.githubusercontent.com/u/6250754?s=200&v=4",
          owner: "0x49E53Fb3d5bf1532fEBAD88a1979E33A94844d1d",
          address: "0x7C78A8fE3eB35A6194760196Ead33063Fae8e33b",
          twitter: "https://twitter.com/ethereum",
          github: "https://github.com/ethereum",
          seedId: null,
          seedHost: null,
        },
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0xCEaB094641905C209CC796FC8037dd9ECc87ca2f",
        members: [
          {
            ethereumAddress: "0x5E813e48a81977c6Fdd565ed5097eb600C73C4f0",
          },
          {
            ethereumAddress: "0x48CB1a8C08066d7Cf200ff01D3beA689A05010f0",
          },
          {
            ethereumAddress: "0x641eDB587867a703E132a4210d63e93C52AA97fF",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x641edb587867a703e132a4210d63e93c52aa97ff",
        id: "0x8152237402e0f194176154c3a6ea1eb99b611482",
        owner: "0xceab094641905c209cc796fc8037dd9ecc87ca2f",
        timestamp: 1628758946,
        registration: {
          domain: "alt-clients.radicle.eth",
          url: "https://app.radicle.network",
          avatar: "https://app.radicle.network/images/alt-clients.png",
          owner: "0xeeE2DF7abdaDe3340954b1bcbE33800190af1fC2",
          address: "0x8152237402E0f194176154c3a6eA1eB99b611482",
          twitter: "radicle",
          github: "radicle-dev",
          seedId: "hyyqpngdoe4x4oto3emfdppbw7sj1pfaghbpmmhz5rqiuqg8uofmeo",
          seedHost: "seed.alt-clients.radicle.xyz",
        },
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0xbc556D111e3df41ABC5b53E148F62E66F84beBf6",
      },
      org: {
        creator: "0xbc556d111e3df41abc5b53e148f62e66f84bebf6",
        id: "0x8cbf44b334fd6cb9a7af6ba0e29345defae247f0",
        owner: "0xbc556d111e3df41abc5b53e148f62e66f84bebf6",
        timestamp: 1629474381,
        registration: {
          domain: "dxdao.radicle.eth",
          url: "https://dxdao.eth.link/",
          avatar:
            "https://gateway.pinata.cloud/ipfs/QmeNS8R873ks9WCEVsSs5gKeSauLb5CgFjTZumHCQFSUbr",
          owner: "0x441428454B614d2bB2782165EF1CEE53da79dd93",
          address: "0x8CBF44b334fD6CB9a7Af6ba0E29345DEFAe247F0",
          twitter: "DXdao_",
          github: null,
          seedId:
            "hydskrqs59bacwb13aibhrfc76k34dcsjodecq19fwxswqacgdkxey@dxdao.lnlhb.com:21726",
          seedHost: null,
        },
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0xe6Ce37cb74b6Ad2d7676A436b529277a97eD4F38",
      },
      org: {
        creator: "0xe6ce37cb74b6ad2d7676a436b529277a97ed4f38",
        id: "0x91309794eff524fa7b3d67b129a107390fac3957",
        owner: "0xe6ce37cb74b6ad2d7676a436b529277a97ed4f38",
        timestamp: 1629524325,
        registration: {
          domain: "jshwi.radicle.eth",
          url: "https://github.com/jshwi",
          avatar: null,
          owner: "0xe6Ce37cb74b6Ad2d7676A436b529277a97eD4F38",
          address: "0x91309794EFF524Fa7B3D67b129a107390faC3957",
          twitter: null,
          github: "jshwi",
          seedId: null,
          seedHost: null,
        },
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0x074CF1766C2Ea57563E6E53cCa730ab3301087FE",
        members: [
          {
            ethereumAddress: "0xf7FDe6e62522c4251Aff03d46A306aa1BA5e04F1",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0xf7fde6e62522c4251aff03d46a306aa1ba5e04f1",
        id: "0xc21600e7abdfee5231a722076745da458f1b9c8d",
        owner: "0x074cf1766c2ea57563e6e53cca730ab3301087fe",
        timestamp: 1626966128,
        registration: {
          domain: "rapidexpedition.radicle.eth",
          url: "https://rapidexpedition.org/",
          avatar:
            "https://ipfs.fleek.co/ipns/rapidExpedition.eth/public/img/rapidExpedition.svg",
          owner: "0xf7FDe6e62522c4251Aff03d46A306aa1BA5e04F1",
          address: "0xc21600e7AbdFee5231A722076745dA458f1b9C8D",
          twitter: null,
          github: "rapidExpedition",
          seedId: null,
          seedHost: null,
        },
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0x05fae9D85485dC6044F1A79EF9a7ac50691b4Aed",
        members: [
          {
            ethereumAddress: "0x52217bAD145A7a80D510f24A9572781D260f33D3",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x52217bad145a7a80d510f24a9572781d260f33d3",
        id: "0xd3c10016cc7f50da751ea06bc78b85143e1cb471",
        owner: "0x05fae9d85485dc6044f1a79ef9a7ac50691b4aed",
        timestamp: 1631795528,
        registration: {
          domain: "morphism.radicle.eth",
          url: null,
          avatar: null,
          owner: "0x52217bAD145A7a80D510f24A9572781D260f33D3",
          address: "0xD3C10016cC7f50Da751ea06BC78b85143E1cB471",
          twitter: null,
          github: null,
          seedId: "hynwx9msmj9674bt1fusuwgjxyo9pcjq6r8jtpx1nxwg1htsohiwz1",
          seedHost: "code.makerlabs.co.uk",
        },
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0x43d2bD33F5fd04306F761D263608EE9d7772ccfA",
        members: [
          {
            ethereumAddress: "0x5F6445Aea67cEECBa204865E42899F2273C82054",
          },
          {
            ethereumAddress: "0x1c4fB2BF4967A9AEe0081E174a7D38b356029a84",
          },
          {
            ethereumAddress: "0xA66A5686D5c3A42C0b6c76FEd05e58C6bc851E9f",
          },
          {
            ethereumAddress: "0x0Baf8fDF6f68737476Ba13CDB3781B29fe71F471",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x0baf8fdf6f68737476ba13cdb3781b29fe71f471",
        id: "0xe22450214b02c2416481ac2d3be51536f7bb1fff",
        owner: "0x43d2bd33f5fd04306f761d263608ee9d7772ccfa",
        timestamp: 1624453662,
        registration: {
          domain: "upstream.radicle.eth",
          url: "https://radicle.xyz/downloads.html",
          avatar:
            "https://res.cloudinary.com/juliendonck/image/upload/v1626346479/icon_2x_bpangt.png",
          owner: "0x0Baf8fDF6f68737476Ba13CDB3781B29fe71F471",
          address: "0xe22450214b02C2416481aC2d3Be51536f7bb1fFf",
          twitter: "radicle",
          github: "radicle-dev",
          seedId: "hyncrnppok8iam6y5oemg4fkumj86mc4wsdiirp83z7tdxchk5dbn6",
          seedHost: "seed.upstream.radicle.xyz",
        },
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0xc0A5Fb55c5799e9dA4d3F2A7Ea13f444e3528222",
        members: [
          {
            ethereumAddress: "0xFEEDC450742AC0D9bB38341D9939449e3270f76F",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x5d2e561a59b5ab7da7ff9551502515ccdc9293d8",
        id: "0x0c9e540936be2da6fa221a1333b763556d781a83",
        owner: "0xc0a5fb55c5799e9da4d3f2a7ea13f444e3528222",
        timestamp: 1626913848,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x0AFe2b28fdD73F20F4F39e25500BB33f524e0655",
      },
      org: {
        creator: "0x0afe2b28fdd73f20f4f39e25500bb33f524e0655",
        id: "0x10eb03b735a0ecf09c9d27780f711f147042337e",
        owner: "0x0afe2b28fdd73f20f4f39e25500bb33f524e0655",
        timestamp: 1631342006,
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0xE604E173d371524A4A523290bD2F0c4DD92bda76",
        members: [
          {
            ethereumAddress: "0x88D01d4354DA0D65C38b0196c4400525A5eebe17",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x88d01d4354da0d65c38b0196c4400525a5eebe17",
        id: "0x308b6a4a3a4e883ab7aaf64d67583f4a45e2d3cd",
        owner: "0xe604e173d371524a4a523290bd2f0c4dd92bda76",
        timestamp: 1628991851,
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0x441428454B614d2bB2782165EF1CEE53da79dd93",
        members: [
          {
            ethereumAddress: "0xEfEdD691224C6aFF392e7B65b594743f2489Fc84",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0xa5aca1577cb8981d512b74da4c95ac7f3be07efc",
        id: "0x34f2c1ac75d0627ab29e367718235d058d5fd88a",
        owner: "0x441428454b614d2bb2782165ef1cee53da79dd93",
        timestamp: 1625499194,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0xB616340a10F14218E5dB19cdAddA686A9557658B",
      },
      org: {
        creator: "0xb616340a10f14218e5db19cdadda686a9557658b",
        id: "0x5a0a2fe16bd95ce30b080a5d9b991b63b462c402",
        owner: "0xb616340a10f14218e5db19cdadda686a9557658b",
        timestamp: 1626943292,
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0xF5512D277A815833ACFC56548382753Bd6F8953c",
        members: [
          {
            ethereumAddress: "0x808A023B72260170c95d831F589A1ae0DCa1e43E",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x808a023b72260170c95d831f589a1ae0dca1e43e",
        id: "0x5a8f8c62af2c11910a2099547ca5885e79707be6",
        owner: "0xf5512d277a815833acfc56548382753bd6f8953c",
        timestamp: 1626367074,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x01623c98F9651095D600F2ecb45dFa1072fC9589",
      },
      org: {
        creator: "0x01623c98f9651095d600f2ecb45dfa1072fc9589",
        id: "0x612a5adf8cf8b655c427a1f70bf656f2f0be5bf2",
        owner: "0x01623c98f9651095d600f2ecb45dfa1072fc9589",
        timestamp: 1628167072,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0xB616340a10F14218E5dB19cdAddA686A9557658B",
      },
      org: {
        creator: "0xb616340a10f14218e5db19cdadda686a9557658b",
        id: "0x74bf3a90fc54f8db5c34de98c188ef98832c6679",
        owner: "0xb616340a10f14218e5db19cdadda686a9557658b",
        timestamp: 1626943313,
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0x6B0F00207D226E4E098839d1b75a1E75dE4B90c0",
        members: [
          {
            ethereumAddress: "0x8873b045d40A458e46E356a96279aE1820a898bA",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x8873b045d40a458e46e356a96279ae1820a898ba",
        id: "0x7b57a4f532a959f52d6e59d193f5bd3ebf949fa8",
        owner: "0x6b0f00207d226e4e098839d1b75a1e75de4b90c0",
        timestamp: 1632672341,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0xD3B282e9880cDcB1142830731cD83f7ac0e1043f",
      },
      org: {
        creator: "0xd3b282e9880cdcb1142830731cd83f7ac0e1043f",
        id: "0x911929f84c12d9dc8ae4b5e2780c9c25c65f87ff",
        owner: "0xd3b282e9880cdcb1142830731cd83f7ac0e1043f",
        timestamp: 1626701789,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0xEA43e87071E7fC452BFc69948bB75Ac4e02eeEBd",
      },
      org: {
        creator: "0xea43e87071e7fc452bfc69948bb75ac4e02eeebd",
        id: "0x96693b10aaded6aa927993bfed707c1fca96d738",
        owner: "0xea43e87071e7fc452bfc69948bb75ac4e02eeebd",
        timestamp: 1630750351,
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0x331C76DFB0a624d4c98830C2aceE0cAb6fBc9E93",
        members: [
          {
            ethereumAddress: "0x912EB1090FD98e506eD4ef0Fc90635D6626dc259",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x912eb1090fd98e506ed4ef0fc90635d6626dc259",
        id: "0xa7fdb6b50a0ba86e2c8ec3c1e6f7978c95ba4160",
        owner: "0x331c76dfb0a624d4c98830c2acee0cab6fbc9e93",
        timestamp: 1629234937,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x536299a4fCF0d1d492596C2C816fc20F3F7Ec557",
      },
      org: {
        creator: "0x536299a4fcf0d1d492596c2c816fc20f3f7ec557",
        id: "0xb6f03b74300aa1462928a56e5a00af91eff7b04b",
        owner: "0x536299a4fcf0d1d492596c2c816fc20f3f7ec557",
        timestamp: 1628903942,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x0dDf7310e3625aB79B6F79991ef56627432185BC",
      },
      org: {
        creator: "0x0ddf7310e3625ab79b6f79991ef56627432185bc",
        id: "0xbbb964397ac42aa91233bca0022bad973f20d22f",
        owner: "0x0ddf7310e3625ab79b6f79991ef56627432185bc",
        timestamp: 1627866420,
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0xab77c7245E4b04E315ebCe6aa8692b1D73775dDa",
        members: [
          {
            ethereumAddress: "0x1C0Aa8cCD568d90d61659F060D1bFb1e6f855A20",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0x1c0aa8ccd568d90d61659f060d1bfb1e6f855a20",
        id: "0xbf33d32ba0b90f2c1db4cf70316024134bde8df5",
        owner: "0xab77c7245e4b04e315ebce6aa8692b1d73775dda",
        timestamp: 1626474422,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x641eDB587867a703E132a4210d63e93C52AA97fF",
      },
      org: {
        creator: "0xefeba186fc1445fb93deaea12ed6f206a4b5d974",
        id: "0xceaa01bd5a428d2910c82bbefe1bc7a8cc6207d9",
        owner: "0x641edb587867a703e132a4210d63e93c52aa97ff",
        timestamp: 1626788188,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x315d35DFC2a50348237E9E999Af8F1C05f1d94B4",
      },
      org: {
        creator: "0x315d35dfc2a50348237e9e999af8f1c05f1d94b4",
        id: "0xd1159ccefd6e15156e5aaab379ae97a20ade62e2",
        owner: "0x315d35dfc2a50348237e9e999af8f1c05f1d94b4",
        timestamp: 1627827179,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0xE76ee3eEfC838C87a0168674b1F5a08553A24946",
      },
      org: {
        creator: "0xe76ee3eefc838c87a0168674b1f5a08553a24946",
        id: "0xd3eae6c65c4a7380832740fa97bf25e66f4fd93c",
        owner: "0xe76ee3eefc838c87a0168674b1f5a08553a24946",
        timestamp: 1626373586,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x5eEe0073017Bd5a3Bc0F031c3Ff84Cf12302C04C",
      },
      org: {
        creator: "0x5eee0073017bd5a3bc0f031c3ff84cf12302c04c",
        id: "0xe5d68364346bd32afd54054d963bf34f6fbefadc",
        owner: "0x5eee0073017bd5a3bc0f031c3ff84cf12302c04c",
        timestamp: 1626380849,
      },
    },
    {
      owner: {
        type: "gnosis-safe",
        address: "0x441428454B614d2bB2782165EF1CEE53da79dd93",
        members: [
          {
            ethereumAddress: "0xEfEdD691224C6aFF392e7B65b594743f2489Fc84",
          },
        ],
        threshold: 1,
      },
      org: {
        creator: "0xefedd691224c6aff392e7b65b594743f2489fc84",
        id: "0xef64b4ad31a2cccea68fb9060f9a4d3634fe6b91",
        owner: "0x441428454b614d2bb2782165ef1cee53da79dd93",
        timestamp: 1629460797,
      },
    },
    {
      owner: {
        type: "wallet",
        address: "0x17e8165f3006C5033d007fcC6A0399a8E71C248b",
      },
      org: {
        creator: "0x17e8165f3006c5033d007fcc6a0399a8e71c248b",
        id: "0xff79e41cee5d5ba093bca6df7a3120afeb5d5353",
        owner: "0x17e8165f3006c5033d007fcc6a0399a8e71c248b",
        timestamp: 1626420693,
      },
    },
  ];

  interface ResolvedOrg {
    org: Org.Org;
    owner: Org.Owner;
  }

  async function fetchOrgs() {
    const unresolvedOrgs = await theGraphApi.getAllOrgs();
    let unsortedOrgs: ResolvedOrg[] = [];
    await Promise.all(
      unresolvedOrgs.map(async org => {
        org.registration = await ensResolver.getCachedRegistrationByAddress(
          org.id
        );
      })
    );

    unsortedOrgs = await Promise.all(
      unresolvedOrgs.map(async org => {
        const owner = await Org.getOwner(org.id);
        return { owner, org };
      })
    );

    resolvedOrgs = await Promise.all(
      unsortedOrgs.sort((a, b) => {
        // Show members with claimed identities first.
        if (!a.org.registration?.domain && b.org.registration?.domain) {
          return 1;
        }
        if (a.org.registration?.domain && !b.org.registration?.domain) {
          return -1;
        }
        return 0;
      })
    );

    console.log(resolvedOrgs);
  }

  // fetchOrgs();
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 2rem auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.5rem;
    margin-top: 2rem;
  }

  .box {
    border: 1px solid var(--color-foreground-level-3);
    border-radius: 0.5rem;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    cursor: pointer;
  }

  .box:hover {
    background-color: var(--color-foreground-level-1);
  }

  .box:active {
    transition: transform 0.1s ease-in-out;
    transform: scale(0.99);
  }

  .title-meta {
    display: flex;
    flex-direction: column;
    text-overflow: ellipsis;
    overflow: hidden;
    gap: 0.5rem;
  }

  .row {
    display: flex;
    align-items: center;
  }

  .row:not(:first-child) {
    margin-top: 0.5rem;
  }
</style>

<ScreenLayout dataCy="network-page">
  <div class="container">
    <h1 style="padding: 0 0.75rem;">Explore orgs</h1>
    <div class="grid">
      {#each resolvedOrgs as { org, owner }}
        <div
          class="box"
          on:click={() =>
            push({
              type: "org",
              params: { address: org.id, view: "projects" },
            })}>
          <header class="row">
            <Avatar
              style="margin-right: 1rem;"
              size="large"
              kind={org.registration?.avatar
                ? { type: "orgImage", url: org.registration.avatar }
                : { type: "orgEmoji", uniqueIdentifier: org.id }} />
            <div class="title-meta">
              <h3 class="typo-overflow-ellipsis">
                {org.registration?.domain ? org.registration.domain : org.id}
              </h3>
              <Badge
                style="align-self: flex-start;"
                caption={owner.type === "wallet"
                  ? "Single signer org"
                  : "Multi signer org"} />
            </div>
          </header>
          <ul class="metadata">
            <li class="row">
              {#if owner.threshold}
                <Icon.Gnosis />
              {:else}
                <Icon.Ethereum />
              {/if}
              <CopyableIdentifier
                style="margin-left: 0.5rem;"
                value={org.id}
                kind="ethAddress"
                name="org address"
                showIcon={false} />
            </li>
            {#if owner.type === "gnosis-safe"}
              <li class="row">
                <Icon.Orgs />
                <p style="margin-left: .5rem;">
                  {owner.threshold} of {owner.members.length} signatures required
                </p>
              </li>
            {/if}
          </ul>
        </div>
      {/each}
    </div>
  </div>
</ScreenLayout>
