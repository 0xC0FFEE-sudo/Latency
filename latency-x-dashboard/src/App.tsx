import { Layout } from "@/components/Layout";
import { Overview } from "@/pages/Overview";
import { TradesPage } from "@/pages/Trades";
import { LogsPage } from "@/pages/Logs";
import { SettingsPage } from "@/pages/Settings";
import { HashRouter as Router, Routes, Route } from "react-router-dom";
import useLocalStorage from "use-local-storage";
import { useEffect } from "react";


function App() {
  const [theme] = useLocalStorage("theme", "light");

  useEffect(() => {
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  return (
      <Router>
        <Layout>
          <Routes>
            <Route path="/" element={<Overview />} />
            <Route path="/trades" element={<TradesPage />} />
            <Route path="/logs" element={<LogsPage />} />
            <Route path="/settings" element={<SettingsPage />} />
          </Routes>
        </Layout>
      </Router>
  )
}

export default App
