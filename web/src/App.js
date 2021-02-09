import './App.css';
import { StatusBadge, AlertText } from './Status';
import Dashboard from './Dashboard';
import Sidebar from './Sidebar';

import { useState, useEffect } from 'react';
import Navbar from\ 'react-bootstrap/Navbar';
import Nav from 'react-bootstrap/Nav';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';

const SERVER_ADDR = "localhost:4242";

function App() {
  const [status, setStatus] = useState({
    data: null,
    isLoaded: false,
    error: null,
  });

  useEffect(() => {
    loadResource("status", setStatus);
    setInterval(() => loadResource("status", setStatus), 5000);
  }, []);

  return (
    <div>
      <Navbar variant="dark" bg="dark" sticky="top" className="p-0 shadow">
        <Navbar.Brand className="me-0 px-3 col-lg-2">cu-ouster</Navbar.Brand>
        <Navbar.Text className="mx-2"><StatusBadge status={status}></StatusBadge></Navbar.Text>
        <Navbar.Text className="mx-2"><AlertText status={status}></AlertText></Navbar.Text>
      </Navbar>
      <Container fluid>
        <Row>
          <Sidebar />
          <Col>
            <Dashboard />
          </Col>
        </Row>
      </Container>
    </div>
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
