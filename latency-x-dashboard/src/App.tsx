import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Layout } from './components/Layout';
import { useWebSocket } from './hooks/useWebSocket';
import { Overview } from './pages/Overview';
import { TradesPage } from './pages/Trades';
import { LogsPage } from './pages/Logs';
import { SettingsPage } from './pages/Settings';

function App() {
  const { isConnected } = useWebSocket('ws://localhost:3000/ws');

  return (
    <Router>
      <Routes>
        <Route path="/" element={<Layout isConnected={isConnected} />}>
          <Route index element={<Overview />} />
          <Route path="trades" element={<TradesPage />} />
          <Route path="logs" element={<LogsPage />} />
          <Route path="settings" element={<SettingsPage />} />
        </Route>
      </Routes>
    </Router>
  );
}

export default App;
