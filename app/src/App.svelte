<script>
  import Projects from './components/Projects.svelte'

  import ApolloClient from 'apollo-boost'
  import { setClient } from 'svelte-apollo'

  const client = new ApolloClient({
    uri: 'http://127.0.0.1:8080/graphql'
  });
  setClient(client)
</script>

<style>
	h1 {
		color: purple;
	}
</style>

<h1>Radicle</h1>
<Projects />
