import './Sidebar.css';
import Nav from 'react-bootstrap/Nav';
import { Home } from 'react-feather';

function Sidebar(props) {
    return <Nav className="sidebar col-lg-2 bg-light">
        <div className="position-sticky pt-3">
            <Nav as="ul" className="flex-column">
                <Nav.Item as="li">
                    <Nav.Link><Home className="feather" />Home</Nav.Link>
                </Nav.Item>
            </Nav>
        </div>
    </Nav>;
}

export default Sidebar;