import Document, {
  Html,
  Head,
  Main,
  NextScript,
  DocumentContext,
} from 'next/document'
import { ApolloProvider } from '@apollo/client'
import { client } from '../utils';

class MyDocument extends Document {
  static async getInitialProps(ctx: DocumentContext) {
    const initialProps = await Document.getInitialProps(ctx);
    return { ...initialProps };
  }

  render() {
    return (
      <ApolloProvider client={client}>
        <Html>
          <Head />
          <body>
          <Main />
          <NextScript />
          </body>
        </Html>
      </ApolloProvider>
    )
  }
}

export default MyDocument