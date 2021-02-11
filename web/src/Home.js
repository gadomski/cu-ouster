import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Card from 'react-bootstrap/Card';
import Loading from './Loading';
import activeAlerts from './Alerts';
import { Link } from 'react-router-dom';
import Alert from 'react-bootstrap/Alert';

function Home(props) {
    return <div>
        <Row>
            <Col><ServerCard status={props.status} serverAddr={props.serverAddr} /></Col>
            <Col><ScannerCard status={props.status} /></Col>
            <Col><AlertsCard status={props.status} /></Col>
        </Row>
    </div>;
}

function ServerCard(props) {
    let border, text;
    if (props.status.isLoaded) {
        border = "success";
        text = `Connected to server at ${props.serverAddr}`;
    } else if (props.status.error) {
        border = "danger";
        text = props.status.error;
    } else {
        border = "light";
        text = <Loading />;
    }
    return <HomeCard border={border} title="Server" subtitle="API server running on the fitlet" text={text} />
}

function ScannerCard(props) {
    let border, text;
    if (props.status.isLoaded) {
        if (props.status.data.is_scanner_connected) {
            border = "success";
            text = `Connected to scanner at ${props.status.data.scanner_addr}`;
        } else {
            border = "danger";
            text = "Not connected to scanner";
        }
    } else {
        border = "light";
        text = <Loading />;
    }
    return <HomeCard title="Scanner" subtitle="OS-1 64 lidar scanner" border={border} text={text} />;
}

function AlertsCard(props) {
    let border, text, suffix;
    if (props.status.isLoaded) {
        if (props.status.data.is_scanner_connected) {
            const alerts = activeAlerts(props.status.data.alerts);
            if (alerts.length == 0) {
                border = "success";
                text = "No active alerts";
            } else {
                border = "danger";
                text = `${alerts.length} active alerts`;
                suffix = <Link to="/alerts" className="btn btn-outline-dark">Go to alerts page</Link>;
            }
        } else {
            border = "danger";
            text = "Not connected to scanner";
        }
    } else {
        border = "light";
        text = <Loading />;
    }
    return <HomeCard title="Alerts" subtitle="Alerts from the scanner" border={border} text={text} suffix={suffix} />;
}

function HomeCard(props) {
    return <Card border={props.border} bg="light">
        <Card.Body>
            <Card.Title>{props.title}</Card.Title>
            <Card.Subtitle className="mb-4 text-muted">{props.subtitle}</Card.Subtitle>
            <Card.Text>
                <Alert variant={props.border}>{props.text}</Alert>
            </Card.Text>
            {props.suffix}
        </Card.Body>
    </Card>
}

export default Home;