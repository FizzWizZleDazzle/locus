use leptos::prelude::*;

#[component]
pub fn TermsOfService() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto px-4 py-12">
            <h1 class="text-4xl font-bold mb-8">"Terms of Service"</h1>
            <p class="text-sm text-gray-500 mb-8">"Effective Date: February 14, 2026"</p>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"1. Acceptance of Terms"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "By accessing or using Locus, you agree to be bound by these Terms of Service and our Privacy Policy. If you do not agree to these terms, please do not use our service."
                </p>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "These terms constitute a legally binding agreement between you and Locus. By creating an account or using the platform, you acknowledge that you have read, understood, and agree to be bound by these terms."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"2. Description of Service"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "Locus is a competitive mathematics learning platform that provides:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Interactive mathematics problems across various topics"</li>
                    <li>"ELO-based rating system for competitive learning"</li>
                    <li>"Leaderboards and progress tracking"</li>
                    <li>"Personalized problem recommendations"</li>
                    <li>"User accounts and profile management"</li>
                </ul>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We reserve the right to modify, suspend, or discontinue any aspect of the service at any time without prior notice."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"3. User Accounts"</h2>

                <h3 class="text-xl font-medium mt-6 mb-3">"3.1 Eligibility"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "You must be at least 13 years old to use Locus. By creating an account, you represent that you meet this age requirement and that all information you provide is accurate and complete."
                </p>

                <h3 class="text-xl font-medium mt-6 mb-3">"3.2 Account Registration"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "To use certain features, you must create an account. You agree to:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Provide accurate and current information"</li>
                    <li>"Maintain the security of your password"</li>
                    <li>"Accept responsibility for all activities under your account"</li>
                    <li>"Notify us immediately of any unauthorized access"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"3.3 Account Security"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "You are responsible for maintaining the confidentiality of your account credentials. Do not share your password with others or allow others to access your account. You agree to notify us immediately of any security breach."
                </p>

                <h3 class="text-xl font-medium mt-6 mb-3">"3.4 Account Termination"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We reserve the right to suspend or terminate your account at any time for violations of these terms, including but not limited to:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Cheating or unfair play"</li>
                    <li>"Creating multiple accounts"</li>
                    <li>"Harassment or abusive behavior"</li>
                    <li>"Violation of applicable laws"</li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"4. User Conduct"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "You agree to use Locus in a fair and respectful manner. Prohibited activities include:"
                </p>

                <h3 class="text-xl font-medium mt-6 mb-3">"4.1 Cheating and Unfair Practices"</h3>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Using automated tools or bots to solve problems"</li>
                    <li>"Sharing or receiving answers to problems"</li>
                    <li>"Creating multiple accounts to manipulate ratings"</li>
                    <li>"Exploiting bugs or vulnerabilities for unfair advantage"</li>
                    <li>"Coordinating with others to artificially inflate ratings"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"4.2 Prohibited Behavior"</h3>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Harassment, bullying, or threatening other users"</li>
                    <li>"Posting offensive, inappropriate, or illegal content"</li>
                    <li>"Attempting to gain unauthorized access to the platform"</li>
                    <li>"Reverse engineering or decompiling platform code"</li>
                    <li>"Interfering with the proper functioning of the service"</li>
                    <li>"Impersonating other users or Locus staff"</li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"5. Intellectual Property"</h2>

                <h3 class="text-xl font-medium mt-6 mb-3">"5.1 Platform Ownership"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "The Locus platform, API, software, design, text, graphics, and logos are the proprietary property of Locus and are protected by copyright and intellectual property laws."
                </p>

                <h3 class="text-xl font-medium mt-6 mb-3">"5.2 Problem Content"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "The mathematical problems and the scripts used to generate them are licensed under the MIT License. This means you are free to use, copy, modify, and distribute the problems and generation scripts, subject to the terms of the MIT License."
                </p>

                <h3 class="text-xl font-medium mt-6 mb-3">"5.3 User Content"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "You retain ownership of your account data and problem-solving history. By using Locus, you grant us a limited license to use this data to provide and improve our service, including displaying your progress and rankings on leaderboards."
                </p>

                <h3 class="text-xl font-medium mt-6 mb-3">"5.4 Usage Restrictions"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "You may not:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Copy, modify, or reverse engineer the platform or API code"</li>
                    <li>"Use the platform infrastructure for commercial purposes without authorization"</li>
                    <li>"Remove copyright notices or proprietary markings from the platform"</li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"6. ELO System and Rankings"</h2>

                <h3 class="text-xl font-medium mt-6 mb-3">"6.1 How Ratings Work"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "Locus uses an ELO-based rating system to track your skill level. Your rating changes based on:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Successfully solving problems (rating increases)"</li>
                    <li>"Failing to solve problems (rating may decrease)"</li>
                    <li>"The difficulty of problems attempted"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"6.2 Fair Play Expectations"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "The integrity of the rating system depends on fair play. We expect all users to:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Solve problems independently without external assistance"</li>
                    <li>"Not manipulate the system through multiple accounts"</li>
                    <li>"Report bugs or rating issues promptly"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"6.3 Rating Adjustments"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We reserve the right to:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Recalculate ratings if errors or cheating are detected"</li>
                    <li>"Reset ratings in cases of system changes or updates"</li>
                    <li>"Remove users from leaderboards for violations"</li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"7. Disclaimers and Limitation of Liability"</h2>

                <h3 class="text-xl font-medium mt-6 mb-3">"7.1 Service Provided \"As Is\""</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "Locus is provided on an \"as is\" and \"as available\" basis. We make no warranties, express or implied, regarding:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Accuracy or completeness of content"</li>
                    <li>"Uninterrupted or error-free service"</li>
                    <li>"Fitness for a particular purpose"</li>
                    <li>"Security of data transmission"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"7.2 Limitation of Liability"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "To the maximum extent permitted by law, Locus shall not be liable for:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Any indirect, incidental, or consequential damages"</li>
                    <li>"Loss of data, profits, or opportunity"</li>
                    <li>"Service interruptions or technical failures"</li>
                    <li>"Actions of other users on the platform"</li>
                </ul>

                <h3 class="text-xl font-medium mt-6 mb-3">"7.3 Educational Use"</h3>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "Locus is an educational tool designed to supplement your mathematics learning. We do not guarantee specific educational outcomes, test scores, or academic success. Your progress depends on your effort and dedication."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"8. Indemnification"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "You agree to indemnify and hold harmless Locus, its affiliates, and personnel from any claims, damages, or expenses arising from:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Your use of the service"</li>
                    <li>"Your violation of these terms"</li>
                    <li>"Your violation of any rights of another party"</li>
                    <li>"Your violation of applicable laws"</li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"9. Changes to Terms"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "We may modify these Terms of Service at any time. When we make significant changes, we will:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Update the effective date at the top of this page"</li>
                    <li>"Notify you via email or platform notification"</li>
                    <li>"Provide a reasonable notice period before changes take effect"</li>
                </ul>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "Your continued use of Locus after changes take effect constitutes acceptance of the modified terms. If you do not agree with the changes, you should discontinue use of the service."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"10. Governing Law and Dispute Resolution"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "These terms are governed by and construed in accordance with applicable laws. Any disputes arising from these terms or your use of Locus shall be resolved through:"
                </p>
                <ul class="list-disc list-inside text-gray-700 space-y-2 ml-4 mb-4">
                    <li>"Good faith negotiations between the parties"</li>
                    <li>"Mediation or arbitration if negotiations fail"</li>
                </ul>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"11. Contact Information"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "If you have questions about these Terms of Service, please contact us at:"
                </p>
                <p class="text-gray-700 leading-relaxed ml-4 mb-4">
                    "Email: support@locusmath.org"
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"12. Severability"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "If any provision of these terms is found to be unenforceable or invalid, that provision shall be limited or eliminated to the minimum extent necessary, and the remaining provisions shall remain in full force and effect."
                </p>
            </section>

            <section class="mb-8">
                <h2 class="text-2xl font-semibold mb-4">"13. Entire Agreement"</h2>
                <p class="text-gray-700 leading-relaxed mb-4">
                    "These Terms of Service, together with our Privacy Policy, constitute the entire agreement between you and Locus regarding your use of the service and supersede any prior agreements."
                </p>
            </section>
        </div>
    }
}
