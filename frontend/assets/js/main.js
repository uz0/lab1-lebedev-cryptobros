import 'regenerator-runtime/runtime';
import { initContract, login, logout, getBros, addBro, removeBro } from './near/utils';

// `nearInitPromise` gets called on page load
window.nearInitPromise = initContract()
  .then(flow)
  .catch(console.error);

function flow() {
  if (window.walletConnection.isSignedIn()) {
    signedInFlow();
  }else{
    signedOutFlow();
  }
}

// Display the signed-out-flow container
function signedOutFlow() {
  document.querySelector('.sign-in').style.display = 'block';
  document.querySelector('.sign-in .btn').onclick = login;
}

function getBroButton(text, color, method) {
  const button = document.createElement('button');
  button.className = 'bro btn btn-primary';
  button.textContent = text;
  button.style.backgroundColor = color;

  button.onclick = async () => {
    document.querySelector('.connections').classList.add('busy');
    try {
      await method();
      await fillBros();
    } finally {
      document.querySelector('.connections').classList.remove('busy');
    }
  };

  return button;
}

async function fillBros() {
  const allBros = await getBros();
  const following = Object.entries(allBros).filter(([bro, status]) => status === 'Following').map(([bro]) => bro);
  const followedBy = Object.entries(allBros).filter(([bro, status]) => status === 'FollowedBy').map(([bro]) => bro);
  const bros = Object.entries(allBros).filter(([bro, status]) => status === 'Bros').map(([bro]) => bro);

  document.querySelectorAll('.bro').forEach((bro) => bro.remove());

  document.querySelector('.following').append(
    ...following.map(
      (bro) => getBroButton(`${bro} (click to unfollow)`, '', () => removeBro(bro))
    )
  );

  document.querySelector('.followedBy').append(
    ...followedBy.map(
      (bro) => getBroButton(`${bro} (click to bro)`, '', () => addBro(bro))
    )
  );

  document.querySelector('.bros').append(
    ...bros.map(
      (bro) => getBroButton(`${bro} 😎 (click to unbro)`, '', () => removeBro(bro))
    )
  );
}

// Displaying the signed in flow container and display counter
function signedInFlow() {
  document.querySelector('.sign-out').style.display = 'block';
  document.querySelector('.sign-out .btn').onclick = logout;

  document.querySelector('.header').textContent = `Cryptobros (feat. ${window.accountId} 😎)!`;
  document.querySelector('.connections').style.display = 'flex';
  document.querySelector('.following button').onclick = async () => {
    const newBro = window.prompt('Enter name of a bro you want to add 😎:', '')?.trim();
    if (!newBro) {
      return;
    }

    try {
      document.querySelector('.connections').classList.add('busy');
      await addBro(newBro);
      await fillBros();
    } finally {
      document.querySelector('.connections').classList.remove('busy');
    }
  };

  void fillBros();
}
