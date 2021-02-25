import Form from 'react-bootstrap/Form';

function Config(props) {
    return <Form onSubmit={e => e.preventDefault()}>
        <Form.Group>
            <Form.Label>Server address</Form.Label>
            <Form.Control onChange={props.onChange} value={props.serverAddr}></Form.Control>
        </Form.Group>
    </Form>;
}

export default Config;