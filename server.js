//codizer(title='Parse JSON Data',description='Parses incoming JSON data and validates the structure',developed_by='Hacı Mert Gökhan')
function authenticateUser(req, res) {
    const { id, title } = req.body;
    res.send({ message: `User ${title} authenticated` });
}

