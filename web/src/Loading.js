import Spinner from 'react-bootstrap/Spinner';

function Loading() {
    return <span><Spinner animation="border" className="mr-2" size="sm" as="span" />Loading...</span>;
}

export default Loading;