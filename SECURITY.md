# Security Policy

## Reporting a Vulnerability

At AVI, we take security issues seriously. We appreciate your efforts to responsibly disclose your findings and will make every effort to acknowledge your contributions.

### How to Report a Security Vulnerability

If you believe you've found a security vulnerability in AVI, please follow these steps:

1. **Do not disclose the vulnerability publicly** (including in GitHub issues)
2. Email your findings to: tiagobernrdo@gmail.com
3. Include the following information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any suggestions for mitigation (if applicable)

### What to Expect

- **Initial Response**: We aim to acknowledge receipt of your report within 48 hours
- **Status Updates**: We will provide regular updates on the progress of addressing the vulnerability
- **Resolution Timeline**: We strive to resolve critical issues within 30 days of verification
- **Disclosure**: We will coordinate with you on the disclosure timeline

## Supported Versions

Only the latest released version and the development branch receive security updates. If you're running an older version, please update to the latest release.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

## Security Measures

### Code Security

- All code is reviewed before merging into the main branch
- We use static analysis tools to identify potential security issues
- The project maintains a comprehensive test suite with security-focused tests

### Dependency Management

- Dependencies are regularly audited for known vulnerabilities
- We use `cargo audit` to scan for vulnerable dependencies
- Critical security updates to dependencies are prioritized

### Secure Development Practices

- We follow the principle of least privilege throughout the codebase
- Authentication and authorization are separated concerns
- All user input is validated and sanitized
- Sensitive data is encrypted in transit and at rest

## External Audit Status

AVI has not yet undergone a formal external security audit. As the project matures, we plan to conduct regular security assessments.

## Best Practices for Deployers

If you're deploying AVI in your environment, consider the following security recommendations:

1. **Run with Minimal Privileges**: Configure AVI to run with the minimum privileges required
2. **Network Security**: Limit network access to the AVI instance
3. **Regular Updates**: Keep your AVI installation up-to-date with the latest releases
4. **Input Validation**: Always validate user inputs in your AviScript skills
5. **Secure Communication**: Use TLS/SSL for all communications with AVI
6. **Authentication**: Implement strong authentication mechanisms for administrative access
7. **Logging**: Enable comprehensive logging for security monitoring

## Disclosure Policy

We believe in responsible disclosure. When we receive a security bug report, we will:

1. Confirm the vulnerability
2. Determine the scope and impact
3. Develop and test a fix
4. Release a patch
5. Publicly disclose the issue after a reasonable time period

## Hall of Fame

We appreciate security researchers who help improve our security. Contributors who responsibly disclose vulnerabilities will be acknowledged in our Security Hall of Fame (coming soon).

## Version History

This security policy was last updated on April 27, 2025.
