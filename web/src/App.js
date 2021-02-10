import './App.css';
import { StatusBadge, AlertText } from './Status';
import Alerts from './Alerts';
import Home from './Home';
import Sidebar from './Sidebar';

import { useState, useEffect } from 'react';
import Navbar from 'react-bootstrap/Navbar';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import { BrowserRouter, Link, Switch, Route } from 'react-router-dom';

const SERVER_ADDR = "localhost:4242";

function App() {
  const [status, setStatus] = useState({
    data: null,
    isLoaded: false,
    error: null,
  });

  useEffect(() => {
    loadResource("status", setStatus);
  }, []);

  return (
    <BrowserRouter>
      <Navbar variant="dark" bg="dark" sticky="top" className="p-0 shadow">
        <Link to="/" className="me-0 px-3 col-lg-2 navbar-brand">cu-ouster</Link>
        <Navbar.Text className="mx-2"><StatusBadge status={status}></StatusBadge></Navbar.Text>
        <Navbar.Text className="mx-2"><AlertText status={status}></AlertText></Navbar.Text>
      </Navbar>
      <Container fluid>
        <Row>
          <Col lg={2}>
            <Sidebar />
          </Col>
          <Col lg={10} className="pt-4 px-4">
            <Switch>
              <Route path="/alerts"><Alerts status={status} /></Route>
              <Route path="/"><Home /></Route>
            </Switch>
          </Col>
        </Row>
      </Container>
    </BrowserRouter>
  );
}


function loadResource(name, setResource) {
  fetch(`http://${SERVER_ADDR}/${name}`)
    .then(result => result.json())
    .then(
      (result) => {
        setResource({
          error: null,
          isLoaded: true,
          data: result
        });
      },
      (error) => {
        setResource({
          error: error,
          isLoaded: false,
          data: null,
        });
      }
    )
}

export default App;
