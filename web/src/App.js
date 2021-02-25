import './App.css';
import { StatusBadge, AlertText } from './Status';
import Alerts from './Alerts';
import Home from './Home';
import Sidebar from './Sidebar';
import Info from './Info';
import Config from './Config';

import { useState, useEffect } from 'react';
import Navbar from 'react-bootstrap/Navbar';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import { BrowserRouter, Link, Switch, Route } from 'react-router-dom';

const SERVER_ADDR = "localhost:4242";
const UPDATE_INTERVAL_MS = 5000;

function App() {
  const [status, setStatus] = useState({
    data: null,
    isLoaded: false,
    error: null,
  });
  const [serverAddr, setServerAddr] = useState(SERVER_ADDR)

  useEffect(() => {
    loadResource(serverAddr, "status", setStatus);
  }, [serverAddr]);
  useEffect(() => {
    const interval = setInterval(() => {
      loadResource(serverAddr, "status", setStatus);
    }, UPDATE_INTERVAL_MS);
    return () => clearInterval(interval);
  }, [serverAddr]);

  function onChangeServerAddr(e) {
    e.preventDefault();
    setServerAddr(e.target.value);
  }

  return (
    <BrowserRouter>
      <Navbar variant="dark" bg="dark" sticky="top" className="p-0 shadow">
        <Link to="/" className="me-0 px-3 col-lg-2 navbar-brand">cu-ouster</Link>
        <Navbar.Text className="mx-2"><StatusBadge status={status}></StatusBadge></Navbar.Text>
        <Navbar.Text className="mx-2"><AlertText status={status}></AlertText></Navbar.Text>
        <Navbar.Text className="mx-2">{serverAddr}</Navbar.Text>
      </Navbar>
      <Container fluid>
        <Row>
          <Sidebar />
          <Col lg={10} md={9} className="ml-sm-auto pt-4 px-4">
            <Switch>
              <Route path="/config"><Config onChange={onChangeServerAddr} serverAddr={serverAddr} /></Route>
              <Route path="/alerts"><Alerts status={status} /></Route>
              <Route path="/info"><Info status={status} /></Route>
              <Route path="/"><Home status={status} serverAddr={serverAddr} /></Route>
            </Switch>
          </Col>
        </Row>
      </Container>
    </BrowserRouter>
  );
}


function loadResource(serverAddr, name, setResource) {
  fetch(`http://${serverAddr}/${name}`)
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
