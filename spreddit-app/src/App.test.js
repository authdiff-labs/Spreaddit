import { render, screen } from '@testing-library/react';
import App from './App';

test('renders Spreaddit heading', () => {
  render(<App />);
  expect(screen.getByText(/Spreddit/i)).toBeInTheDocument();
});

test('renders post link action', () => {
  render(<App />);
  expect(screen.getByRole('button', { name: /post new link/i })).toBeInTheDocument();
});
