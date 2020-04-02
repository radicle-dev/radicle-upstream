import { writable, Writable } from "svelte/store"

// Perhaps it'd make sense to use project.id for routing
interface CurrentProjectStore {
  name: string
}

const createCurrentProjectStore = () => {
  const { subscribe, set, update }: Writable<CurrentProjectStore | null> = writable(null)

  return {
    subscribe,
    update: (name: string) => update(() => { return { name } })
  }
}

export const currentProjectName = createCurrentProjectStore()
