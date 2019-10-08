import {A} from 'hookrouter';
import React from 'react';

const Navigation = () => {
  return(
    <ul>
      <li><A href="/">/</A></li>
      <li><A href="/join-network">JoinNetworkScreen</A></li>
      <li><A href="/projects">Projects</A></li>
      <li><A href="/projects/1">First project</A></li>
      <li><A href="/projects/2">Second project</A></li>
      <li><A href="/wrong">Wrong</A></li>
    </ul>
  )
}

export default Navigation;
