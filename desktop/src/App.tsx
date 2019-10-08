import {useRoutes} from 'hookrouter';
import React from 'react';

import JoinNetworkScreen from './screens/JoinNetworkScreen';
import ProjectsScreen from './screens/ProjectsScreen';
import ProjectScreen from './screens/ProjectScreen';
import NotFoundScreen from './screens/NotFoundScreen';

import ScreenLayout from './layouts/ScreenLayout';

const routes = {
    '/': () => <ProjectsScreen />,
    '/join-network': () => <JoinNetworkScreen />,
    '/projects': () => <ProjectsScreen />,
    '/projects/:id': ({id}: any) => <ProjectScreen id={id} />
};

const App = () => {
    const routeResult = useRoutes(routes);

    if (routeResult) {
      return <ScreenLayout>{routeResult}</ScreenLayout>
    } else {
      return <NotFoundScreen />
    }
}

export default App;
