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
    let variant, text;
    if (props.status.isLoaded) {
        variant = "success";
        text = `Connected to server at ${props.serverAddr}`;
    } else if (props.status.error) {
        variant = "danger";
        text = props.status.error.toString();
    } else {
        variant = "secondary";
        text = <Loading />;
    }
    return <HomeCard variant={variant} title="Server" subtitle="API server running on the fitlet" text={text} />
}

function ScannerCard(props) {
    let variant, text, suffix;
    if (props.status.isLoaded) {
        if (props.status.data.is_scanner_connected) {
            variant = "success";
            text = `Connected to scanner at ${props.status.data.scanner_addr}`;
            suffix = <Link to="/info" className="btn btn-outline-dark">Info</Link>;
        } else {
            variant = "danger";
            text = "Not connected to scanner";
        }
    } else {
        variant = "secondary";
        text = <Loading />;
    }
    return <HomeCard title="Scanner" subtitle="OS-1 64 lidar scanner" variant={variant} text={text} suffix={suffix} />;
}

function AlertsCard(props) {
    let variant, text, suffix;
    if (props.status.isLoaded) {
        if (props.status.data.is_scanner_connected) {
            const alerts = props.status.data.alerts.log.filter(a => a.active);
            if (alerts.length == 0) {
                variant = "success";
                text = "No active alerts";
            } else {
                variant = "danger";
                text = `${alerts.length} active alerts`;
                suffix = <Link to="/alerts" className="btn btn-outline-dark">Alerts</Link>;
            }
        } else {
            variant = "danger";
            text = "Not connected to scanner";
        }
    } else {
        variant = "secondary";
        text = <Loading />;
    }
    return <HomeCard title="Alerts" subtitle="Alerts from the scanner" variant={variant} text={text} suffix={suffix} />;
}

function HomeCard(props) {
    return <Card className={`alert-${props.variant}`}>
        <Card.Body>
            <Card.Title>{props.title}</Card.Title>
            <Card.Subtitle className="mb-4 text-muted">{props.subtitle}</Card.Subtitle>
            <Card.Text>
                {props.text}
            </Card.Text>
        </Card.Body>
        {props.suffix && <div><hr /><Card.Body className="pt-1 text-center">{props.suffix}</Card.Body></div>}
    </Card>
}

export default Home;