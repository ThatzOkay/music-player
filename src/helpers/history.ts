import { Location, NavigateFunction } from 'react-router-dom'

interface iHistory {
  navigate: NavigateFunction | null
  location: Location | null
}

// custom history object to allow navigation outside react components
export const history: iHistory = {
  navigate: null,
  location: null
}
