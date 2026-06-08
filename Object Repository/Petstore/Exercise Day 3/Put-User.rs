<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Put-User</name>
   <tag></tag>
   <elementGuidId>82938211-a6a7-4b73-b74d-8b385cfe9f26</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: 205,\n  \&quot;username\&quot;: \&quot;${param_body_username}\&quot;,\n  \&quot;firstName\&quot;: \&quot;Devan Updated\&quot;,\n  \&quot;lastName\&quot;: \&quot;QA\&quot;,\n  \&quot;email\&quot;: \&quot;devan@mail.com\&quot;,\n  \&quot;password\&quot;: \&quot;${param_body_password}\&quot;,\n  \&quot;phone\&quot;: \&quot;08123456789\&quot;,\n  \&quot;userStatus\&quot;: 0\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9cadce0d-caa7-409a-8682-a375781a139e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>b1b7878d-977f-46d0-973e-b85b26c2c17e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.1.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user/${param_url_username}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>4807a4c3-2420-4c88-8851-0bf75042268b</id>
      <name>New Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Schema/SchemeExercise.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.current_username</defaultValue>
      <description></description>
      <id>eb76c61b-68c4-4ef1-9a35-4961456d0628</id>
      <masked>false</masked>
      <name>param_url_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.current_username</defaultValue>
      <description></description>
      <id>fe26dbd5-9a92-4ef2-b01f-d5ac69cefbcf</id>
      <masked>false</masked>
      <name>param_body_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.current_password</defaultValue>
      <description></description>
      <id>f1517b9b-a52d-473e-a0b7-4fb1091acaa6</id>
      <masked>false</masked>
      <name>param_body_password</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'code', 200)
WS.verifyElementPropertyValue(response, 'type', &quot;unknown&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
