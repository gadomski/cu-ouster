import { Link } from 'react-router-dom';

function StatusBadge(props) {
    let variant = "danger";
    let text = "Not connected";
    if (props.status.isLoaded) {
        const data = props.status.data;
        if (data.is_scanner_connected) {
            const status = data.metadata.sensor_info.status;
            text = status;
            switch (status) {
                case "UNCONFIGURED":
                case "ERROR":
                    variant = "danger";
                    break;
                case "UPDATING":
                case "INITIALIZING":
                    variant = "warning";
                    break;
                case "RUNNING":
                    variant = "success";
                    break;
            }
        } else {
            variant = "danger";
            text = "No scanner";
        }
    }
    const className = `badge badge-${variant}`;
    return <Link to="/alerts" className={className}>{text}</Link>;
}

function AlertText(props) {
    if (props.status.isLoaded && props.status.data.is_scanner_connected) {
        const alerts = props.status.data.metadata.alerts.log;
        if (alerts.length > 0) {
            const alert = alerts[0];
            return alert.msg;
        } else {
            return null;
        }
    } else {
        return null;
    }
}

export { StatusBadge, AlertText };