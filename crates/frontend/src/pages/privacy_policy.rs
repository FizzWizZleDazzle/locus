use leptos::prelude::*;

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto px-4 py-12">
            <h1 class="text-4xl font-bold mb-8">"Privacy Policy"</h1>
            <p class="text-sm text-gray-500 mb-8">"Effective Date: February 14, 2026"</p>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"1. Introduction"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "Welcome to Locus. We are committed to protecting your privacy and handling your data in an open and transparent manner. This Privacy Policy explains how we collect, use, store, and protect your personal information when you use our competitive mathematics learning platform."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"2. Information We Collect"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We collect the following types of information:"
                </p>

                <h3 class="text-xl font-medium mt-6 mb-3">"2.1 Account Information"</h3>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Email address"</li>
                    <li>"Username"</li>
                    <li>"Password (stored securely using bcrypt hashing)"</li>
                    <li>"Account creation date"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"2.2 OAuth Authentication Data"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "If you choose to sign in with third-party providers, we collect:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Google OAuth: Email address, profile information, and OAuth tokens"</li>
                    <li>"GitHub OAuth: Username, email address, and OAuth tokens"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"2.3 Usage Data"</h3>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Problems attempted and solved"</li>
                    <li>"Problem-solving history and timestamps"</li>
                    <li>"ELO rating and ranking information"</li>
                    <li>"Topic preferences and practice patterns"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"2.4 Technical Data"</h3>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Session information and cookies"</li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"3. How We Use Your Information"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We use your information for the following purposes:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4">
                    <li>"Authentication and account management"</li>
                    <li>"Providing personalized problem recommendations"</li>
                    <li>"Calculating and displaying ELO ratings and leaderboards"</li>
                    <li>"Tracking your progress and learning patterns"</li>
                    <li>"Improving our service and user experience"</li>
                    <li>"Sending important service-related communications"</li>
                    <li>"Preventing fraud and ensuring platform security"</li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"4. Data Storage and Security"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We take data security seriously and implement industry-standard measures to protect your information:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"All data is stored in secure PostgreSQL databases"</li>
                    <li>"Passwords are hashed using bcrypt before storage"</li>
                    <li>"OAuth tokens are securely stored and encrypted"</li>
                    <li>"Data transmission is protected using HTTPS encryption"</li>
                    <li>"Access to user data is restricted to authorized personnel only"</li>
                </ul>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "While we strive to protect your data, no method of transmission over the internet is 100% secure. We cannot guarantee absolute security but continuously work to improve our security measures."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"5. Third-Party Services"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "Locus integrates with the following third-party services:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Google OAuth: For authentication (governed by Google's Privacy Policy)"</li>
                    <li>"GitHub OAuth: For authentication (governed by GitHub's Privacy Policy)"</li>
                </ul>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We do not share your personal information with third parties except as required for authentication services or as required by law."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"6. Your Rights"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "You have the following rights regarding your personal data:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Access: You can request a copy of your personal data"</li>
                    <li>"Correction: You can update or correct your information through your account settings"</li>
                    <li>"Deletion: You can request deletion of your account and associated data"</li>
                    <li>"Portability: You can request an export of your data in a machine-readable format"</li>
                    <li>"Objection: You can object to certain types of data processing"</li>
                </ul>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "To exercise these rights, please contact us using the information provided in Section 8."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"7. Cookies and Tracking"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We use cookies and similar technologies for:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Session management and authentication"</li>
                    <li>"Remembering your preferences"</li>
                    <li>"Analytics to improve our service"</li>
                </ul>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "You can control cookies through your browser settings, but disabling cookies may affect your ability to use certain features of Locus."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"8. Contact Information"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "If you have questions or concerns about this Privacy Policy or how we handle your data, please contact us at:"
                </p>
                <p class="text-gray-700 leading-relaxed ml-4 mb-4">
                    "Email: privacy@locusmath.org"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"9. Changes to This Privacy Policy"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We may update this Privacy Policy from time to time to reflect changes in our practices or for legal, operational, or regulatory reasons. When we make significant changes, we will notify you by:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Posting the updated policy on this page with a new effective date"</li>
                    <li>"Sending an email notification to your registered email address"</li>
                </ul>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We encourage you to review this Privacy Policy periodically to stay informed about how we protect your information."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"10. Children's Privacy"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "Locus is intended for users aged 13 and older. We do not knowingly collect personal information from children under 13. If we become aware that we have collected information from a child under 13, we will take steps to delete such information."
                </p>
            </section>
        </div>
    }
}
