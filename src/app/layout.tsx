import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Sambook",
  description: "세 단어로 짧은 이야기를 쓰는 한글 글쓰기 놀이터",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ko">
      <body>{children}</body>
    </html>
  );
}

