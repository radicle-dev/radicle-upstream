import React from 'react';
import Navigation from '../components/Navitagion';

const ScreenLayout: React.SFC = ({children}) => {
  return(
    <>
      <Navigation />
      {children}
    </>
  )
}

export default ScreenLayout;
