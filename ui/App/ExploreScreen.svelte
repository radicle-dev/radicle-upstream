<!--
 Copyright © 2021 The Radicle Upstream Contributors

 This file is part of radicle-upstream, distributed under the GPLv3
 with Radicle Linking Exception. For full terms see the included
 LICENSE file.
-->
<script lang="typescript">
  import * as theGraphApi from "ui/src/org/theGraphApi";
  import * as ensResolver from "ui/src/org/ensResolver";

  import ScreenLayout from "ui/App/ScreenLayout.svelte";

  let orgs: theGraphApi.Org[] = [
    {
      creator: "0x5d2e561a59b5ab7da7ff9551502515ccdc9293d8",
      id: "0x0c9e540936be2da6fa221a1333b763556d781a83",
      owner: "0xc0a5fb55c5799e9da4d3f2a7ea13f444e3528222",
      timestamp: 1626913848,
    },
    {
      creator: "0x0afe2b28fdd73f20f4f39e25500bb33f524e0655",
      id: "0x10eb03b735a0ecf09c9d27780f711f147042337e",
      owner: "0x0afe2b28fdd73f20f4f39e25500bb33f524e0655",
      timestamp: 1631342006,
    },
    {
      creator: "0x88d01d4354da0d65c38b0196c4400525a5eebe17",
      id: "0x308b6a4a3a4e883ab7aaf64d67583f4a45e2d3cd",
      owner: "0xe604e173d371524a4a523290bd2f0c4dd92bda76",
      timestamp: 1628991851,
    },
    {
      creator: "0xa5aca1577cb8981d512b74da4c95ac7f3be07efc",
      id: "0x34f2c1ac75d0627ab29e367718235d058d5fd88a",
      owner: "0x441428454b614d2bb2782165ef1cee53da79dd93",
      timestamp: 1625499194,
    },
    {
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
    {
      creator: "0xb616340a10f14218e5db19cdadda686a9557658b",
      id: "0x5a0a2fe16bd95ce30b080a5d9b991b63b462c402",
      owner: "0xb616340a10f14218e5db19cdadda686a9557658b",
      timestamp: 1626943292,
    },
    {
      creator: "0x808a023b72260170c95d831f589a1ae0dca1e43e",
      id: "0x5a8f8c62af2c11910a2099547ca5885e79707be6",
      owner: "0xf5512d277a815833acfc56548382753bd6f8953c",
      timestamp: 1626367074,
    },
    {
      creator: "0x01623c98f9651095d600f2ecb45dfa1072fc9589",
      id: "0x612a5adf8cf8b655c427a1f70bf656f2f0be5bf2",
      owner: "0x01623c98f9651095d600f2ecb45dfa1072fc9589",
      timestamp: 1628167072,
    },
    {
      creator: "0xb616340a10f14218e5db19cdadda686a9557658b",
      id: "0x74bf3a90fc54f8db5c34de98c188ef98832c6679",
      owner: "0xb616340a10f14218e5db19cdadda686a9557658b",
      timestamp: 1626943313,
    },
    {
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
    {
      creator: "0x8873b045d40a458e46e356a96279ae1820a898ba",
      id: "0x7b57a4f532a959f52d6e59d193f5bd3ebf949fa8",
      owner: "0x6b0f00207d226e4e098839d1b75a1e75de4b90c0",
      timestamp: 1632672341,
    },
    {
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
    {
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
    {
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
    {
      creator: "0xd3b282e9880cdcb1142830731cd83f7ac0e1043f",
      id: "0x911929f84c12d9dc8ae4b5e2780c9c25c65f87ff",
      owner: "0xd3b282e9880cdcb1142830731cd83f7ac0e1043f",
      timestamp: 1626701789,
    },
    {
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
    {
      creator: "0xea43e87071e7fc452bfc69948bb75ac4e02eeebd",
      id: "0x96693b10aaded6aa927993bfed707c1fca96d738",
      owner: "0xea43e87071e7fc452bfc69948bb75ac4e02eeebd",
      timestamp: 1630750351,
    },
    {
      creator: "0x912eb1090fd98e506ed4ef0fc90635d6626dc259",
      id: "0xa7fdb6b50a0ba86e2c8ec3c1e6f7978c95ba4160",
      owner: "0x331c76dfb0a624d4c98830c2acee0cab6fbc9e93",
      timestamp: 1629234937,
    },
    {
      creator: "0x536299a4fcf0d1d492596c2c816fc20f3f7ec557",
      id: "0xb6f03b74300aa1462928a56e5a00af91eff7b04b",
      owner: "0x536299a4fcf0d1d492596c2c816fc20f3f7ec557",
      timestamp: 1628903942,
    },
    {
      creator: "0x0ddf7310e3625ab79b6f79991ef56627432185bc",
      id: "0xbbb964397ac42aa91233bca0022bad973f20d22f",
      owner: "0x0ddf7310e3625ab79b6f79991ef56627432185bc",
      timestamp: 1627866420,
    },
    {
      creator: "0x1c0aa8ccd568d90d61659f060d1bfb1e6f855a20",
      id: "0xbf33d32ba0b90f2c1db4cf70316024134bde8df5",
      owner: "0xab77c7245e4b04e315ebce6aa8692b1d73775dda",
      timestamp: 1626474422,
    },
    {
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
    {
      creator: "0xefeba186fc1445fb93deaea12ed6f206a4b5d974",
      id: "0xceaa01bd5a428d2910c82bbefe1bc7a8cc6207d9",
      owner: "0x641edb587867a703e132a4210d63e93c52aa97ff",
      timestamp: 1626788188,
    },
    {
      creator: "0x315d35dfc2a50348237e9e999af8f1c05f1d94b4",
      id: "0xd1159ccefd6e15156e5aaab379ae97a20ade62e2",
      owner: "0x315d35dfc2a50348237e9e999af8f1c05f1d94b4",
      timestamp: 1627827179,
    },
    {
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
    {
      creator: "0xe76ee3eefc838c87a0168674b1f5a08553a24946",
      id: "0xd3eae6c65c4a7380832740fa97bf25e66f4fd93c",
      owner: "0xe76ee3eefc838c87a0168674b1f5a08553a24946",
      timestamp: 1626373586,
    },
    {
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
    {
      creator: "0x5eee0073017bd5a3bc0f031c3ff84cf12302c04c",
      id: "0xe5d68364346bd32afd54054d963bf34f6fbefadc",
      owner: "0x5eee0073017bd5a3bc0f031c3ff84cf12302c04c",
      timestamp: 1626380849,
    },
    {
      creator: "0xefedd691224c6aff392e7b65b594743f2489fc84",
      id: "0xef64b4ad31a2cccea68fb9060f9a4d3634fe6b91",
      owner: "0x441428454b614d2bb2782165ef1cee53da79dd93",
      timestamp: 1629460797,
    },
    {
      creator: "0x17e8165f3006c5033d007fcc6a0399a8e71c248b",
      id: "0xff79e41cee5d5ba093bca6df7a3120afeb5d5353",
      owner: "0x17e8165f3006c5033d007fcc6a0399a8e71c248b",
      timestamp: 1626420693,
    },
  ];

  // async function fetchOrgs() {
  //   orgs = await theGraphApi.getAllOrgs();
  //   orgs.map(async org => {
  //     org.registration = await ensResolver.getCachedRegistrationByAddress(
  //       org.id
  //     );
  //   });
  //   console.log(orgs);
  // }

  // fetchOrgs();
</script>

<style>
  .container {
    max-width: var(--content-max-width);
    margin: 4rem auto;
    min-width: var(--content-min-width);
    padding: 0 var(--content-padding);
  }
</style>

<ScreenLayout dataCy="network-page">
  <div class="container">
    <h1>Explore</h1>
    <div class="content">
      {#each orgs as org}
        <p>{org.id}</p>
      {/each}
    </div>
  </div>
</ScreenLayout>
