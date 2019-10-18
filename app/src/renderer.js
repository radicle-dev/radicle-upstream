import React from "react"
import ReactDOM from "react-dom"
import {hot} from 'react-hot-loader/root'

import App from './App.js'

const Container = hot(App);

ReactDOM.render(<Container />, document.getElementById("app"))
