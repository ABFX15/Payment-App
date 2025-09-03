import { WalletButton } from '../solana/solana-provider'
import { PaymentCreate, PaymentProgram, PaymentProgramExplorerLink } from './payment-ui'
import { AppHero } from '../app-hero'
import { useWalletUi } from '@wallet-ui/react'

export default function PaymentFeature() {
  const { account } = useWalletUi()

  if (!account) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="hero py-[64px]">
          <div className="hero-content text-center">
            <WalletButton />
          </div>
        </div>
      </div>
    )
  }

  return (
    <div>
      <AppHero title="Payment" subtitle={'Run the program by clicking the "Run program" button.'}>
        <p className="mb-6">
          <PaymentProgramExplorerLink />
        </p>
        <PaymentCreate />
      </AppHero>
      <PaymentProgram />
    </div>
  )
}
