fetch('components/navbar.html')
.then(response => response.text())
.then(data => {
    console.log(data);
    document.getElementById('item-navbar').innerHTML = data;
});
