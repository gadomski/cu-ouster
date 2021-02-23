import Loading from './Loading';
import { Alert as BootstrapAlert } from 'react-bootstrap';

function Alerts(props) {
    let alerts;
    if (!props.status.isLoaded) {
        alerts = <Loading />;
    } else if (!props.status.data.is_scanner_connected) {
        alerts = <BootstrapAlert variant="danger">Not connected to scanner</BootstrapAlert>;
    } else {
        alerts = activeAlerts(props.status.data.alerts);
        if (alerts.length === 0) {
            alerts = [<BootstrapAlert key={-1} variant="success">No active alerts!</BootstrapAlert>];
        }
    }
    return (
        <div>
            <h2 className="border-bottom mb-4 pb-3">Alerts</h2>
            {alerts}
        </div>
    )
}

function Alert(props) {
    const alert = props.alert;
    let variant;
    if (alert.level === "ERROR") {
        variant = "danger";
    } else if (alert.level === "WARNING") {
        variant = "warning";
    }
    return <BootstrapAlert variant={variant}>
        <BootstrapAlert.Heading>{alert.category}</BootstrapAlert.Heading>
        <p>{alert.msg}</p>
        {alert.msg_verbose && <p>{alert.msg_verbose}</p>}
    </BootstrapAlert>
}

function activeAlerts(alerts) {
    return alerts.log.filter(a => a.active).map(a => <Alert alert={a} key={a.cursor} />);
}

export default Alerts;