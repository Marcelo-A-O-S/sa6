fetch('components/navbar.html')
.then(response => response.text())
.then(data => {
    document.getElementById('item-navbar').innerHTML = data;
});
