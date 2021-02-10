import './Sidebar.css';
import Nav from 'react-bootstrap/Nav';
import { Home, AlertTriangle } from 'react-feather';
import { NavLink } from 'react-router-dom';

function Sidebar(props) {
    return (
        <Nav className="sidebar col-lg-2 bg-light">
            <div className="position-sticky pt-3">
                <Nav as="ul" className="flex-column">
                    <Nav.Item as="li">
                        <NavLink to="/" exact className="nav-link"><Home className="feather" />Home</NavLink>
                        <NavLink to="/alerts" className="nav-link"><AlertTriangle className="feather" />Alerts</NavLink>
                    </Nav.Item>
                </Nav>
            </div>
        </Nav>
    );
}

export default Sidebar;