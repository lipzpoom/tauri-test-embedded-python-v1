import json
import sys
import urllib.parse as urlparse
from datetime import datetime, timedelta
from typing import List, Dict, Any
from http.server import HTTPServer, BaseHTTPRequestHandler
import os

class TradingHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        """Handle GET requests"""
        parsed_path = urlparse.urlparse(self.path)
        
        if parsed_path.path == '/trading-history':
            try:
                response_data = get_trading_history_data()
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.send_header('Access-Control-Allow-Origin', '*')
                self.end_headers()
                self.wfile.write(json.dumps(response_data).encode())
            except Exception as e:
                self.send_error(500, str(e))
        
        elif parsed_path.path == '/health':
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(json.dumps({"status": "ok"}).encode())
        
        else:
            self.send_error(404, 'Not Found')
    
    def do_OPTIONS(self):
        """Handle CORS preflight requests"""
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()
    
    def log_message(self, format, *args):
        """Override to suppress default logging"""
        pass

def load_env():
    """Load environment variables from .env file if it exists"""
    env_path = os.path.join(os.path.dirname(__file__), '.env')
    if os.path.exists(env_path):
        with open(env_path, 'r') as f:
            for line in f:
                if '=' in line and not line.startswith('#'):
                    key, value = line.strip().split('=', 1)
                    os.environ[key] = value

def get_trading_history_data() -> dict:
    """
    Get trading history data and return as dictionary.
    This is a mock implementation - replace with actual trading API calls.
    """
    try:
        # Load environment variables
        load_env()
        
        # Mock trading history data
        trading_data = [
            {
                "id": "1",
                "symbol": "BTCUSDT",
                "side": "BUY",
                "quantity": "0.001",
                "price": "45000.00",
                "timestamp": (datetime.now() - timedelta(hours=2)).isoformat(),
                "status": "FILLED"
            },
            {
                "id": "2", 
                "symbol": "ETHUSDT",
                "side": "SELL",
                "quantity": "0.05",
                "price": "3200.00",
                "timestamp": (datetime.now() - timedelta(hours=1)).isoformat(),
                "status": "FILLED"
            },
            {
                "id": "3",
                "symbol": "ADAUSDT", 
                "side": "BUY",
                "quantity": "100.00",
                "price": "0.45",
                "timestamp": datetime.now().isoformat(),
                "status": "PENDING"
            }
        ]
        
        return {
            "success": True,
            "data": trading_data,
            "total": len(trading_data)
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
            "data": []
        }

def get_trading_history() -> str:
    """
    Legacy function for compatibility - returns JSON string
    """
    return json.dumps(get_trading_history_data())

def start_server(port=8765):
    """Start HTTP server"""
    server = HTTPServer(('localhost', port), TradingHandler)
    print(f"Python sidecar server started on http://localhost:{port}")
    print("Available endpoints:")
    print("  /health - Health check")
    print("  /trading-history - Get trading history")
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down server...")
        server.shutdown()

if __name__ == "__main__":
    # Check if running as sidecar (with port argument)
    if len(sys.argv) > 1:
        try:
            port = int(sys.argv[1])
            start_server(port)
        except ValueError:
            print("Invalid port number")
            sys.exit(1)
    else:
        # Legacy mode - just print JSON for compatibility
        result = get_trading_history()
        print(result)