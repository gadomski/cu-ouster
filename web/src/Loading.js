import Spinner from 'react-bootstrap/Spinner';

function Loading() {
    return <p>
        <Spinner animation="border" className="mr-2" size="sm" />
        Loading...
    </p>;
}

export default Loading;