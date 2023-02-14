
export const nightMode = () => {
  const className = 'night-mode';
  var date = new Date();
  var hour = date.getHours();

  if ((hour >= 0 && hour <= 6) || hour === 23) {
    document.body.classList.add(className);
  }
}

