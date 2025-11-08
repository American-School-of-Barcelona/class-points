from flask import Flask, redirect, request, session, url_for, render_template_string
import requests

app = Flask(__name__)
API = "http://localhost:3000"

# -------------------------
# Templates
# -------------------------
home_page = """
<h2>Welcome {{user['name']}}</h2>
<p>Points: {{user['points']}}</p>
<a href="/users">View all users</a><br>
<a href="/logout">Logout</a>
"""

users_page = """
<h2>Users</h2>
<ul>
{% for u in users %}
  <li>{{u['name']}} - Points: {{u['points']}}</li>
{% endfor %}
</ul>
<a href="/">Back</a>
"""

login_page = """
<h2>Login with ASBCS</h2>
<a href="{{login_url}}">Login</a>
"""

# -------------------------
# Helper function to call the API
# -------------------------
def api_request(method, endpoint, token=None, **kwargs):
    headers = {}
    if token:
        # Send JWT token as Bearer token for authentication
        headers["Authorization"] = f"Bearer {token}"
    # All API endpoints are under /api
    return requests.request(method, f"{API}/api{endpoint}", headers=headers, **kwargs)

# -------------------------
# Routes
# -------------------------
@app.route("/")
def index():
    """Home page: shows authenticated user info"""
    token = session.get("token")
    if not token:
        return redirect(url_for("login"))
    # Get authenticated user info from API
    r = api_request("GET", "/users/authenticated", token=token)
    if r.status_code != 200:
        # Invalid or expired token, redirect to login
        return redirect(url_for("login"))
    user = r.json()
    return render_template_string(home_page, user=user)

@app.route("/login")
def login():
    """Redirect user to ASBCS login page"""
    callback = url_for("auth", _external=True)  # where ASBCS should redirect with token
    issuer = "myflaskapp"  # name of this app
    login_url = f"{API}/login?callback={callback}&issuer={issuer}"
    return render_template_string(login_page, login_url=login_url)

@app.route("/auth/")
def auth():
    """Callback endpoint to receive token from ASBCS"""
    token = request.args.get("token")
    if not token:
        return "No token received", 400
    session["token"] = token  # store token in Flask session for future API calls
    return redirect(url_for("index"))

@app.route("/logout")
def logout():
    """Log the user out by clearing the session"""
    session.pop("token", None)
    return redirect(url_for("login"))

@app.route("/users")
def users():
    """Display a list of all users"""
    token = session.get("token")
    r = api_request("GET", "/users/list", token=token)
    users = r.json().get("students", [])
    return render_template_string(users_page, users=users)

if __name__ == "__main__":
    app.run(debug=True)