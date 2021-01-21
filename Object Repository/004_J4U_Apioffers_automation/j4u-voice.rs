<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>j4u-voice</name>
   <tag></tag>
   <elementGuidId>0d64d463-493d-4a2c-8dec-ed9ff1fc0bb2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{  \&quot;username\&quot; : \&quot;comj4u\&quot;,\n \&quot;password\&quot; : \&quot;j4u@456\&quot;,\n \&quot;MSISDN\&quot; : \&quot;243814444589\&quot;,\n \&quot;Category\&quot;: \&quot;Voice\&quot;,\n \&quot;Channel\&quot; : \&quot;WEB\&quot;,\n \&quot;Language\&quot; : \&quot;en\&quot;,\n \&quot;RefNum\&quot; : \&quot;APP1528376639790\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${j4u_Apioffer_Url}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_Url</defaultValue>
      <description></description>
      <id>422852f4-73fc-45be-b97d-abb6aff25e93</id>
      <masked>false</masked>
      <name>j4u_Apioffer_Url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_target_msisdn</defaultValue>
      <description></description>
      <id>096bf0f1-488e-45eb-a3a1-a4dc4bc0cdab</id>
      <masked>false</masked>
      <name>j4u_Apioffer_target_msisdn</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.j4u_Apioffer_random_msisdn</defaultValue>
      <description></description>
      <id>e51cfce5-2d23-483b-801a-808992e80c82</id>
      <masked>false</masked>
      <name>j4u_Apioffer_random_msisdn</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
