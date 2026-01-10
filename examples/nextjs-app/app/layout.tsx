import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'Axiom Next.js Example',
  description: 'Example Next.js application using Axiom client',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
