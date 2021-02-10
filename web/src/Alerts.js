import Loading from './Loading';
import { Alert as BootstrapAlert } from 'react-bootstrap';

function Alerts(props) {
    if (!props.status.isLoaded) {
        return <Loading />;
    }
    let alerts = props.status.data.alerts.log.filter(a => a.active).map(a => <Alert alert={a} key={a.cursor} />);
    if (alerts.length === 0) {
        alerts = [<Alert key={-1} variant="success">No active alerts!</Alert>];
    }
    return (
        <div>
            <h2 className="border-bottom mb-4 pb-2">Alerts</h2>
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

export default Alerts;