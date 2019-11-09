import './static/styles/main.scss';

import('./frontend/pkg')
  .then(app => app.start())
  .catch(console.error);
