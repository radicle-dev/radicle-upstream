open Particle.Color;

module Logo = {
  [@react.component]
  let make = () =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <circle cx="12" cy="12" r="11" stroke="#28333D" strokeWidth="2" />
    </svg>;
};

module Close = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="34"
      height="34"
      viewBox="0 0 34 34"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M25.4853 8.5147C25.0948 8.12418 24.4616 8.12418 24.0711 8.5147L17 15.5858L9.92893 8.5147C9.53841 8.12418 8.90524 8.12418 8.51472 8.5147C8.12419 8.90522 8.12419 9.53839 8.51472 9.92891L15.5858 17L8.51472 24.071C8.12419 24.4616 8.12419 25.0947 8.51472 25.4853C8.90524 25.8758 9.53841 25.8758 9.92893 25.4853L17 18.4142L24.0711 25.4853C24.4616 25.8758 25.0948 25.8758 25.4853 25.4853C25.8758 25.0947 25.8758 24.4616 25.4853 24.071L18.4142 17L25.4853 9.92891C25.8758 9.53839 25.8758 8.90522 25.4853 8.5147Z"
        fill={hexOfColor(color)}
      />
    </svg>;
};

module Plus = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M12 4C11.4477 4 11 4.44772 11 5V11H5C4.44772 11 4 11.4477 4 12C4 12.5523 4.44772 13 5 13H11V19C11 19.5523 11.4477 20 12 20C12.5523 20 13 19.5523 13 19V13H19C19.5523 13 20 12.5523 20 12C20 11.4477 19.5523 11 19 11H13V5C13 4.44772 12.5523 4 12 4Z"
        fill={hexOfColor(color)}
      />
    </svg>;
};

module Important = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M19.0711 4.92893C17.1957 3.05357 14.6522 2 12 2C9.34784 2 6.8043 3.05357 4.92893 4.92893C3.05357 6.8043 2 9.34784 2 12C2 14.6522 3.05357 17.1957 4.92893 19.0711C6.8043 20.9464 9.34784 22 12 22C14.6522 22 17.1957 20.9464 19.0711 19.0711C20.9464 17.1957 22 14.6522 22 12C22 9.34784 20.9464 6.8043 19.0711 4.92893ZM10.9393 17.5607C11.2207 17.842 11.6022 18 12 18C12.3978 18 12.7794 17.842 13.0607 17.5607C13.342 17.2794 13.5 16.8978 13.5 16.5C13.5 16.1022 13.342 15.7207 13.0607 15.4394C12.7794 15.158 12.3978 15 12 15C11.6022 15 11.2207 15.158 10.9393 15.4394C10.658 15.7207 10.5 16.1022 10.5 16.5C10.5 16.8978 10.658 17.2794 10.9393 17.5607ZM11 12.1C11.12 13.3 12.87 13.3 13 12.1L13.5 7.10001C13.514 6.96054 13.4986 6.81967 13.4546 6.68656C13.4107 6.55344 13.3393 6.43105 13.245 6.32733C13.1507 6.2236 13.0356 6.14086 12.9073 6.08447C12.7789 6.02809 12.6402 5.99931 12.5 6.00001H11.5C11.3598 5.99931 11.2211 6.02809 11.0927 6.08447C10.9644 6.14086 10.8493 6.2236 10.7551 6.32733C10.6608 6.43105 10.5893 6.55344 10.5454 6.68656C10.5015 6.81967 10.486 6.96054 10.5 7.10001L11 12.1Z"
        fill={hexOfColor(color)}
      />
    </svg>;
};

module CloseSmall = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        d="M8 8L16 16"
        stroke={hexOfColor(color)}
        strokeWidth="2"
        strokeLinecap="round"
      />
      <path
        d="M8 16L16 8"
        stroke={hexOfColor(color)}
        strokeWidth="2"
        strokeLinecap="round"
      />
    </svg>;
};

module Info = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M19.0711 4.92893C17.1957 3.05357 14.6522 2 12 2C9.34784 2 6.8043 3.05357 4.92893 4.92893C3.05357 6.8043 2 9.34784 2 12C2 14.6522 3.05357 17.1957 4.92893 19.0711C6.8043 20.9464 9.34784 22 12 22C14.6522 22 17.1957 20.9464 19.0711 19.0711C20.9464 17.1957 22 14.6522 22 12C22 9.34784 20.9464 6.8043 19.0711 4.92893ZM10.2603 11.5174C10.427 11.6841 10.6531 11.7778 10.8889 11.7778H11.4844L10.0356 16.8711C9.99856 17.003 9.99256 17.1417 10.018 17.2763C10.0435 17.4109 10.0997 17.5378 10.1823 17.6471C10.2649 17.7564 10.3717 17.8451 10.4943 17.9063C10.6168 17.9676 10.7519 17.9996 10.8889 18H12.6667C12.9024 18 13.1285 17.9064 13.2952 17.7397C13.4619 17.573 13.5556 17.3469 13.5556 17.1111C13.5556 16.8754 13.4619 16.6493 13.2952 16.4826C13.1285 16.3159 12.9024 16.2222 12.6667 16.2222H12.0711L13.52 11.1289C13.557 10.997 13.563 10.8583 13.5375 10.7237C13.5121 10.5891 13.4558 10.4622 13.3732 10.3529C13.2906 10.2436 13.1838 10.1549 13.0613 10.0937C12.9387 10.0324 12.8037 10.0004 12.6667 10H10.8889C10.6531 10 10.427 10.0937 10.2603 10.2603C10.0936 10.427 10 10.6531 10 10.8889C10 11.1246 10.0936 11.3507 10.2603 11.5174ZM11.7239 8.27614C11.9739 8.52619 12.313 8.66667 12.6667 8.66667C13.0203 8.66667 13.3594 8.52619 13.6095 8.27614C13.8595 8.02609 14 7.68696 14 7.33333C14 6.97971 13.8595 6.64057 13.6095 6.39052C13.3594 6.14048 13.0203 6 12.6667 6C12.313 6 11.9739 6.14048 11.7239 6.39052C11.4738 6.64057 11.3333 6.97971 11.3333 7.33333C11.3333 7.68696 11.4738 8.02609 11.7239 8.27614Z"
        fill={hexOfColor(color)}
      />
    </svg>;
};

module Check = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        d="M7 12L11 16L18 9"
        stroke={hexOfColor(color)}
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>;
};

module Search = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        d="M18.2929 19.7071C18.6834 20.0976 19.3166 20.0976 19.7071 19.7071C20.0976 19.3166 20.0976 18.6834 19.7071 18.2929L18.2929 19.7071ZM14.2929 15.7071L18.2929 19.7071L19.7071 18.2929L15.7071 14.2929L14.2929 15.7071Z"
        fill={hexOfColor(color)}
      />
      <circle
        cx="11"
        cy="11"
        r="5"
        stroke={hexOfColor(color)}
        strokeWidth="2"
      />
    </svg>;
};

module Graph = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M12 8C12.5523 8 13 7.55228 13 7C13 6.44772 12.5523 6 12 6C11.4477 6 11 6.44772 11 7C11 7.55228 11.4477 8 12 8ZM15 7C15 8.30622 14.1652 9.41746 13 9.82929V13H16C17.6569 13 19 14.3431 19 16V17.5858L19.2929 17.2929C19.6834 16.9024 20.3166 16.9024 20.7071 17.2929C21.0976 17.6834 21.0976 18.3166 20.7071 18.7071L18.7071 20.7071C18.3166 21.0976 17.6834 21.0976 17.2929 20.7071L15.2929 18.7071C14.9024 18.3166 14.9024 17.6834 15.2929 17.2929C15.6834 16.9024 16.3166 16.9024 16.7071 17.2929L17 17.5858V16C17 15.4477 16.5523 15 16 15H12H8C7.44772 15 7 15.4477 7 16V17.5858L7.29289 17.2929C7.68342 16.9024 8.31658 16.9024 8.70711 17.2929C9.09763 17.6834 9.09763 18.3166 8.70711 18.7071L6.70711 20.7071C6.31658 21.0976 5.68342 21.0976 5.29289 20.7071L3.29289 18.7071C2.90237 18.3166 2.90237 17.6834 3.29289 17.2929C3.68342 16.9024 4.31658 16.9024 4.70711 17.2929L5 17.5858V16C5 14.3431 6.34315 13 8 13H11V9.82929C9.83481 9.41746 9 8.30622 9 7C9 5.34315 10.3431 4 12 4C13.6569 4 15 5.34315 15 7Z"
        fill={hexOfColor(color)}
      />
    </svg>;
};

module Inbox = {
  [@react.component]
  let make = (~color=Gray, ~notificationColor=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M12 3.5C12.5523 3.5 13 3.94772 13 4.5V12.5858L15.2929 10.2929C15.6834 9.90237 16.3166 9.90237 16.7071 10.2929C17.0976 10.6834 17.0976 11.3166 16.7071 11.7071L12.7078 15.7064L12.7005 15.7136C12.5208 15.8901 12.2746 15.9992 12.003 16L12 16L11.997 16C11.8625 15.9996 11.7343 15.9727 11.6172 15.9241C11.502 15.8764 11.3938 15.8063 11.2995 15.7136L11.2922 15.7064L7.29289 11.7071C6.90237 11.3166 6.90237 10.6834 7.29289 10.2929C7.68342 9.90237 8.31658 9.90237 8.70711 10.2929L11 12.5858V4.5C11 3.94772 11.4477 3.5 12 3.5ZM5 14C5 13.4477 4.55228 13 4 13C3.44772 13 3 13.4477 3 14V18C3 19.6569 4.34315 21 6 21H18C19.6569 21 21 19.6569 21 18V14C21 13.4477 20.5523 13 20 13C19.4477 13 19 13.4477 19 14V18C19 18.5523 18.5523 19 18 19H6C5.44772 19 5 18.5523 5 18V14Z"
        fill={hexOfColor(color)}
      />
      <circle cx="20" cy="4" r="4" fill={hexOfColor(notificationColor)} />
    </svg>;
};

module Wallet = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M4 5H18V8H11C8.79086 8 7 9.79086 7 12C7 14.2091 8.79086 16 11 16H18V19H4L4 5ZM20 5V8C21.1046 8 22 8.89543 22 10V14C22 15.1046 21.1046 16 20 16V19C20 20.1046 19.1046 21 18 21H4C2.89543 21 2 20.1046 2 19V5C2 3.89543 2.89543 3 4 3H18C19.1046 3 20 3.89543 20 5ZM18 10H20V14H18H11C9.89543 14 9 13.1046 9 12C9 10.8954 9.89543 10 11 10H18ZM11 11C10.4477 11 10 11.4477 10 12C10 12.5523 10.4477 13 11 13H13C13.5523 13 14 12.5523 14 12C14 11.4477 13.5523 11 13 11H11Z"
        fill={hexOfColor(color)}
      />
    </svg>;
};

module Back = {
  [@react.component]
  let make = (~color=Gray) =>
    <svg
      width="16"
      height="16"
      viewBox="0 0 16 16"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <path
        d="M10 4L6 8L10 12"
        stroke={hexOfColor(color)}
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>;
};

module SadFace = {
  [@react.component]
  let make = (~style=?, ~color=Gray) =>
    <svg
      className=?style
      width="48"
      height="48"
      viewBox="0 0 48 48"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <circle
        cx="24"
        cy="24"
        r="23"
        stroke={hexOfColor(color)}
        strokeWidth="2"
      />
      <path
        d="M33.3688 34.8269C30.457 33.6208 27.3361 33 24.1844 33C21.0327 33 17.9118 33.6208 15 34.8269"
        stroke={hexOfColor(color)}
        strokeWidth="2"
      />
      <rect
        x="10.8719"
        y="18.5361"
        width="6"
        height="2"
        transform="rotate(-10 10.8719 18.5361)"
        fill="#90A0AF"
      />
      <rect
        x="31.2192"
        y="17.4941"
        width="6"
        height="2"
        transform="rotate(10 31.2192 17.4941)"
        fill={hexOfColor(color)}
      />
    </svg>;
};

module PersonAvatarPlaceholder = {
  [@react.component]
  let make = () =>
    <svg
      width="36"
      height="36"
      viewBox="0 0 36 36"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <circle cx="18" cy="18" r="18" fill="url(#paint0_radial)" />
      <defs>
        <radialGradient
          id="paint0_radial"
          cx="0"
          cy="0"
          r="1"
          gradientUnits="userSpaceOnUse"
          gradientTransform="translate(18 18) rotate(90) scale(55.5)">
          <stop stopColor="#E074CB" />
          <stop offset="1" stopColor="#6E41E0" />
        </radialGradient>
      </defs>
    </svg>;
};

module ProjectAvatarPlaceholder = {
  [@react.component]
  let make = () =>
    <svg
      width="64"
      height="64"
      viewBox="0 0 64 64"
      fill="none"
      xmlns="http://www.w3.org/2000/svg">
      <rect width="64" height="64" rx="2" fill="url(#paint0_radial)" />
      <defs>
        <radialGradient
          id="paint0_radial"
          cx="0"
          cy="0"
          r="1"
          gradientUnits="userSpaceOnUse"
          gradientTransform="translate(32 32) rotate(90) scale(98.6667)">
          <stop stopColor="#E074CB" />
          <stop offset="1" stopColor="#6E41E0" />
        </radialGradient>
      </defs>
    </svg>;
};
