const express = require('express')
const app = express()
const port = 8080

app.get('/', async (req, res) => {
    let response = await (await fetch("http://127.0.0.1:8000/api/v2/ability/")).json()
    let abilities = response.results;
    abilities.sort((a,b) => (a.name > b.name) ? 1 : (a.name < b.name) ? -1 : 0)
    res.send(abilities)
})

app.listen(port, () => {
    console.log(`App listening on port ${port}`)
})
