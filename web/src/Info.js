import Loading from './Loading';
import Table from 'react-bootstrap/Table'

function Info(props) {
    if (props.status.isLoaded) {
        const sensorInfo = props.status.data.sensor_info;
        const sensorInfoRows = Object.keys(sensorInfo).map((k, i) => <tr key={i}><th>{k}</th><td>{sensorInfo[k]}</td></tr>);
        return <div>
            <h2>Sensor info</h2>
            <Table className="col-md-6"><tbody>{sensorInfoRows}</tbody></Table>
        </div>;
    } else {
        return <Loading />
    }
}

export default Info;