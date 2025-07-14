import React from 'react';
import ReactDOM from 'react-dom/client';
import SendMessage from './SendMessage';
import GetHistory from './GetHistory';
import logo from './logo.svg';
import './index.css';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
root.render(
  <React.StrictMode>
    <div className="App">
      <img src={logo} className="logo"/>
      <div className="column-layout">
        <div className="column"><SendMessage /></div>
        <div className="column"><GetHistory /></div>
      </div>
    </div>
  </React.StrictMode>
);