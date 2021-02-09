import Badge from 'react-bootstrap/Badge';

function StatusBadge(props) {
    if (props.status.isLoaded) {
        const data = props.status.data;
        if (data.is_scanner_connected) {
            const status = data.sensor_info.status;
            let variant = "success";
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
            return <Badge variant={variant}>{status}</Badge>;
        } else {
            return <Badge variant="warning">No scanner</Badge>;
        }
    } else {
        return <Badge variant="danger">Not connected</Badge>;
    }
}

function AlertText(props) {
    if (props.status.isLoaded && props.status.data.is_scanner_connected) {
        const alerts = props.status.data.alerts.log;
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