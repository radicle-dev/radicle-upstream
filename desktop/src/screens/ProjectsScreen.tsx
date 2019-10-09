import React from 'react';
import { useQuery } from '@apollo/react-hooks';
import { gql } from 'apollo-boost';

const PROJECTS = gql`
  {
    projects {
      address
      description
      name
      imgUrl
    }
  }
`;

const ProjectsScreen = () => {
  const { loading, error, data } = useQuery(PROJECTS);

  console.log(error)
  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error:(</p>;

  return data.projects.map(({ address, description, name, imgUrl }: any) => (
    <div key={address}>
      <p>
        {address} {description} {name} {imgUrl}
      </p>
    </div>
  ));
}

export default ProjectsScreen;
